use crate::components::dashboard::{CpuUsage, MemoryUsage};
use whtop_common::models::api::ProcessInfo;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ProcessListProps {
    pub process_list: Vec<ProcessInfo>,
    pub total_memory: u64,
}

#[function_component(ProcessList)]
pub fn process_list(props: &ProcessListProps) -> Html {
    // let progress = props.cpu_stats.usage / 100.0;
    let process_rows = props.process_list.iter().map(|process| {
        html! {
            <div class={"process-list-row"}>
                <div class={"process-list-row-pid"}>
                    {&process.pid}
                    {
                        if let Some(parent_pid) = process.parent_pid.as_ref() {
                            format!(" ({parent_pid})")
                        } else {
                            String::new()
                        }
                    }
                </div>
                <div class={"process-list-row-name"}>
                    {&process.name}
                </div>
                <div class={"process-list-row-cpu"}>
                    <CpuUsage
                        cpu_name={process.name.clone()}
                        cpu_frequency={None}
                        cpu_usage={process.cpu}
                    />
                </div>
                <div class={"process-list-row-memory"}>
                    <MemoryUsage
                        memory_total={props.total_memory}
                        memory_available={props.total_memory.saturating_sub(process.virtual_memory)}
                        memory_used={process.memory}
                    />
                </div>
            </div>
        }
    });
    html! {
        <div class={"process-list"}>
            { for process_rows }
        </div>
    }
}
