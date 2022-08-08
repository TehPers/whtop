use crate::components::dashboard::{CpuUsage, MemoryUsage};
use futures::join;
use gloo::{net::http::Request, timers::callback::Interval};
use serde::de::DeserializeOwned;
use whtop_common::models::api::{GetCpuResponse, GetMemoryResponse};
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
        // Invalidate the response every second
        let link = ctx.link().clone();
        let interval = Interval::new(1000, move || {
            link.send_message(DashboardMessage::InvalidateResponse)
        });

        Dashboard {
            _interval: interval,
            state: DashboardState::Uninitialized,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DashboardMessage::InvalidateResponse => {
                ctx.link().send_future(update_stats());
                false
            }
            DashboardMessage::RequestError(error) => {
                self.state = DashboardState::Error(error);
                true
            }
            DashboardMessage::UpdateStats {
                memory_stats,
                cpu_stats,
            } => {
                self.state = DashboardState::Ready {
                    memory_stats,
                    cpu_stats,
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
                memory_stats,
                cpu_stats,
            } => {
                html! {
                    <>
                        <h2>{"Memory"}</h2>
                        <section class={"memory"}>
                            <MemoryUsage memory_stats={memory_stats.clone()} />
                        </section>
                        <h2>{"CPU"}</h2>
                        <section class={"cpu"}>
                            <CpuUsage
                                cpu_name={"Average"}
                                cpu_stats={cpu_stats.global.clone()}
                            />
                            {
                                for cpu_stats.cpus.iter().map(|cpu| {
                                    html! {
                                        <CpuUsage
                                            cpu_name={cpu.name.clone()}
                                            cpu_stats={cpu.inner.clone()}
                                            radius={30.0}
                                        />
                                    }
                                })
                            }
                        </section>
                        <h2>{"Processes"}</h2>
                        <section class={"processes"}>
                        </section>
                    </>
                }
            }
            DashboardState::Error(error) => html! {
                <p>{format!("{error:?}")}</p>
            },
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
        memory_stats: GetMemoryResponse,
        cpu_stats: GetCpuResponse,
    },
    RequestError(gloo::net::Error),
}

#[derive(Debug)]
enum DashboardState {
    Uninitialized,
    Ready {
        memory_stats: GetMemoryResponse,
        cpu_stats: GetCpuResponse,
    },
    Error(gloo::net::Error),
}

async fn get_stats<T>(endpoint: &str) -> Result<T, gloo::net::Error>
where
    T: DeserializeOwned,
{
    Request::get(endpoint).send().await?.json().await
}

async fn update_stats() -> DashboardMessage {
    macro_rules! try_stats {
        ($val:expr) => {
            match $val {
                Ok(val) => val,
                Err(error) => return DashboardMessage::RequestError(error),
            }
        };
    }

    // Fetch stats
    let memory_url = "http://localhost:8081/memory";
    let cpu_url = "http://localhost:8081/cpu";
    let (memory_stats, cpu_stats) = join!(get_stats(memory_url), get_stats(cpu_url));

    // Error handling
    let memory_stats: GetMemoryResponse = try_stats!(memory_stats);
    let mut cpu_stats: GetCpuResponse = try_stats!(cpu_stats);

    // Update CPU global frequency
    cpu_stats.global.frequency = cpu_stats
        .cpus
        .iter()
        .map(|cpu| cpu.inner.frequency)
        .sum::<u64>()
        .checked_div(cpu_stats.cpus.len() as u64)
        .unwrap_or(0);

    DashboardMessage::UpdateStats {
        memory_stats,
        cpu_stats,
    }
}
