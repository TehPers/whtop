use crate::{
    components::dashboard::{CpuUsage, MemoryUsage, ProcessList},
    contexts::HttpClient,
};
use anyhow::Context as _;
use futures::{join, FutureExt};
use gloo::net::http::Request;
use serde::de::DeserializeOwned;
use std::rc::Rc;
use whtop_common::models::api::{GetCpuResponse, GetMemoryResponse, GetProcessesResponse};
use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Clone, Debug)]
struct DashboardState {
    errors: Vec<Rc<anyhow::Error>>,
    memory_stats: GetMemoryResponse,
    cpu_stats: GetCpuResponse,
    process_stats: GetProcessesResponse,
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    // Get HTTP client
    let client = use_context::<HttpClient>();
    let Some(client) = client else {
        gloo::console::error!("failed to get http client");
        return Html::default();
    };

    // Update state every 2 secs
    let state = use_state(|| None);
    use_interval(
        {
            let state = state.clone();
            move || {
                let client = client.clone();
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let new_state = update_state(client.clone(), state.as_ref()).await;
                    state.set(Some(new_state));
                });
            }
        },
        2000,
    );

    if let Some(state) = state.as_ref() {
        render_state(state)
    } else {
        html! {
            <p>{"Loading..."}</p>
        }
    }
}

fn render_state(state: &DashboardState) -> Html {
    html! {
        <main class="dashboard">
            {
                if !state.errors.is_empty() {
                    html! {
                        <>
                            <h2 class={"errors"}>{"Errors"}</h2>
                            <section class={"errors"}>
                                <p>{"One or more errors occurred while fetching system information:"}</p>
                                {
                                    for state.errors.iter().map(|error| {
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
                    Html::default()
                }
            }
            <h2>{"Memory"}</h2>
            <section class={"memory"}>
                <MemoryUsage
                    memory_total={state.memory_stats.total}
                    memory_used={state.memory_stats.used}
                    memory_available={state.memory_stats.available}
                />
            </section>
            <h2>{"CPU"}</h2>
            <section class={"cpu"}>
                <CpuUsage
                    cpu_name={"Average"}
                    cpu_usage={state.cpu_stats.global.usage}
                    cpu_frequency={state.cpu_stats.global.frequency}
                />
                {
                    for state.cpu_stats.cpus.iter().map(|cpu| {
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
                    process_list={state.process_stats.processes.clone()}
                    total_memory={state.memory_stats.total}
                />
            </section>
        </main>
    }
}

async fn update_state(client: HttpClient, last_state: Option<&DashboardState>) -> DashboardState {
    // Fetch stats
    const BASE_URL: &str = "/api/system";
    let memory_url = format!("{BASE_URL}/memory");
    let cpu_url = format!("{BASE_URL}/cpu");
    let processes_url = format!("{BASE_URL}/processes");
    let (memory_stats, cpu_stats, process_stats) = join!(
        get_stats(client.clone(), &memory_url)
            .map(|stats| stats.context("failed to get memory stats")),
        get_stats(client.clone(), &cpu_url).map(|stats| stats.context("failed to get CPU stats")),
        get_stats(client.clone(), &processes_url)
            .map(|stats| stats.context("failed to get process stats")),
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

    let (prev_memory_stats, prev_cpu_stats, prev_process_stats) = match &last_state {
        Some(DashboardState {
            memory_stats,
            cpu_stats,
            process_stats,
            ..
        }) => (Some(memory_stats), Some(cpu_stats), Some(process_stats)),
        _ => (None, None, None),
    };

    macro_rules! try_stats {
        ($result:expr, $else:expr, $errors:expr) => {
            match $result {
                Ok(stats) => stats,
                Err(error) => {
                    $errors.push(Rc::new(error));
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

    DashboardState {
        errors,
        memory_stats,
        cpu_stats,
        process_stats,
    }
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
