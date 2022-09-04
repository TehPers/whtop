use crate::{
    components::dashboard::{CpuUsage, MemoryUsage, ProcessList},
    contexts::HttpClient,
};
use anyhow::Context as _;
use futures::join;
use gloo::{net::http::Request, timers::callback::Interval};
use serde::de::DeserializeOwned;
use whtop_common::models::api::{GetCpuResponse, GetMemoryResponse, GetProcessesResponse};
use yew::prelude::*;

#[derive(Debug)]
pub struct Dashboard {
    _interval: Interval,
    state: DashboardState,
}

impl Component for Dashboard {
    type Message = DashboardMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Invalidate the response every 2 seconds
        let link = ctx.link().clone();
        let interval = Interval::new(2000, move || {
            link.send_message(DashboardMessage::InvalidateResponse)
        });

        Dashboard {
            _interval: interval,
            state: DashboardState::Uninitialized,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (client, _handle) = match ctx.link().context::<HttpClient>(Callback::noop()) {
            Some(client) => client,
            None => {
                gloo::console::error!("failed to get http client");
                return false;
            }
        };
        match msg {
            DashboardMessage::InvalidateResponse => {
                ctx.link().send_future(update_stats(client));
                false
            }
            DashboardMessage::UpdateStats {
                memory_stats,
                cpu_stats,
                process_stats,
            } => {
                let (prev_memory_stats, prev_cpu_stats, prev_process_stats) = match &self.state {
                    DashboardState::Ready {
                        memory_stats,
                        cpu_stats,
                        process_stats,
                        ..
                    } => (Some(memory_stats), Some(cpu_stats), Some(process_stats)),
                    _ => (None, None, None),
                };

                macro_rules! try_stats {
                    ($result:expr, $else:expr, $errors:expr) => {
                        match $result {
                            Ok(stats) => stats,
                            Err(error) => {
                                $errors.push(error);
                                $else
                            }
                        }
                    };
                }
                let mut errors = Vec::with_capacity(3);
                let memory_stats = try_stats!(
                    memory_stats,
                    prev_memory_stats.cloned().unwrap_or_default(),
                    errors
                );
                let cpu_stats = try_stats!(
                    cpu_stats,
                    prev_cpu_stats.cloned().unwrap_or_default(),
                    errors
                );
                let process_stats = try_stats!(
                    process_stats,
                    prev_process_stats.cloned().unwrap_or_default(),
                    errors
                );

                self.state = DashboardState::Ready {
                    errors,
                    memory_stats,
                    cpu_stats,
                    process_stats,
                };
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let contents = match &self.state {
            DashboardState::Uninitialized => {
                html! {
                    <p>{"Loading..."}</p>
                }
            }
            DashboardState::Ready {
                errors,
                memory_stats,
                cpu_stats,
                process_stats,
            } => {
                html! {
                    <>
                        {
                            if !errors.is_empty() {
                                html! {
                                    <>
                                        <h2 class={"errors"}>{"Errors"}</h2>
                                        <section class={"errors"}>
                                            <p>{"One or more errors occurred while fetching system information:"}</p>
                                            {
                                                for errors.iter().map(|error| {
                                                    html! {
                                                        <>
                                                            <pre>
                                                                {format!("{error:?}")}
                                                            </pre>
                                                            <hr />
                                                        </>
                                                    }
                                                })
                                            }
                                        </section>
                                    </>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <h2>{"Memory"}</h2>
                        <section class={"memory"}>
                            <MemoryUsage
                                memory_total={memory_stats.total}
                                memory_used={memory_stats.used}
                                memory_available={memory_stats.available}
                            />
                        </section>
                        <h2>{"CPU"}</h2>
                        <section class={"cpu"}>
                            <CpuUsage
                                cpu_name={"Average"}
                                cpu_usage={cpu_stats.global.usage}
                                cpu_frequency={cpu_stats.global.frequency}
                            />
                            {
                                for cpu_stats.cpus.iter().map(|cpu| {
                                    html! {
                                        <CpuUsage
                                            cpu_name={cpu.name.clone()}
                                            cpu_usage={cpu.inner.usage}
                                            cpu_frequency={cpu.inner.frequency}
                                        />
                                    }
                                })
                            }
                        </section>
                        <h2>{"Processes"}</h2>
                        <section class={"processes"}>
                            <ProcessList
                                process_list={process_stats.processes.clone()}
                                total_memory={memory_stats.total}
                            />
                        </section>
                    </>
                }
            }
        };

        html! {
            <main class={"dashboard"}>
                {contents}
            </main>
        }
    }
}

#[derive(Debug)]
pub enum DashboardMessage {
    InvalidateResponse,
    UpdateStats {
        memory_stats: anyhow::Result<GetMemoryResponse>,
        cpu_stats: anyhow::Result<GetCpuResponse>,
        process_stats: anyhow::Result<GetProcessesResponse>,
    },
}

#[derive(Debug)]
enum DashboardState {
    Uninitialized,
    Ready {
        errors: Vec<anyhow::Error>,
        memory_stats: GetMemoryResponse,
        cpu_stats: GetCpuResponse,
        process_stats: GetProcessesResponse,
    },
}

async fn get_stats<T>(client: HttpClient, endpoint: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let request = Request::get(endpoint);
    client
        .send(request)
        .await?
        .inner()
        .json()
        .await
        .context("error parsing response")
}

async fn update_stats(client: HttpClient) -> DashboardMessage {
    // Fetch stats
    const BASE_URL: &str = "/api/system";
    let memory_url = format!("{BASE_URL}/memory");
    let cpu_url = format!("{BASE_URL}/cpu");
    let processes_url = format!("{BASE_URL}/processes");
    let (memory_stats, cpu_stats, process_stats) = join!(
        get_stats(client.clone(), &memory_url),
        get_stats(client.clone(), &cpu_url),
        get_stats(client.clone(), &processes_url)
    );

    // Update CPU global frequency
    let cpu_stats = cpu_stats.map(|mut stats: GetCpuResponse| {
        stats.global.frequency = stats
            .cpus
            .iter()
            .map(|cpu| cpu.inner.frequency)
            .sum::<u64>()
            .checked_div(stats.cpus.len() as u64)
            .unwrap_or(0);
        stats
    });

    DashboardMessage::UpdateStats {
        memory_stats,
        cpu_stats,
        process_stats,
    }
}
