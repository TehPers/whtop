use crate::components::Meter;
use whtop_common::models::api::GetMemoryResponse;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct MemoryUsageProps {
    pub memory_stats: GetMemoryResponse,
}

#[function_component(MemoryUsage)]
pub fn memory_usage(props: &MemoryUsageProps) -> Html {
    let used = format_unit(props.memory_stats.used);
    let allocated = format_unit(props.memory_stats.total - props.memory_stats.available);
    let total = format_unit(props.memory_stats.total);
    let progress = props.memory_stats.used as f64 / props.memory_stats.total as f64;
    html! {
        <div class={"memory-usage"}>
            <div class={"memory-usage-used"}>{used}{" used"}</div>
            <div class={"memory-usage-allocated"}>{allocated}{" allocated"}</div>
            <div class={"memory-usage-total"}>{total}{" available"}</div>
            <div class={"memory-usage-bar"}>
                <Meter {progress} />
            </div>
        </div>
    }
}

fn format_unit(kilobytes: u64) -> String {
    if kilobytes > 1000000 {
        format!("{:.2} GB", kilobytes as f64 / 1000000.0)
    } else if kilobytes > 1000 {
        format!("{:.2} MB", kilobytes as f64 / 1000.0)
    } else {
        format!("{} KB", kilobytes)
    }
}
