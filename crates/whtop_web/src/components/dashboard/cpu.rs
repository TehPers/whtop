use crate::components::Meter;
use whtop_common::models::api::GlobalCpuInfo;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CpuUsageProps {
    pub cpu_name: String,
    pub cpu_stats: GlobalCpuInfo,
    #[prop_or_else(|| 120.0)]
    pub radius: f64,
}

#[function_component(CpuUsage)]
pub fn cpu_usage(props: &CpuUsageProps) -> Html {
    let progress = props.cpu_stats.usage / 100.0;
    html! {
        <div class={"cpu-usage"}>
            <div class={"cpu-usage-name"}>{&props.cpu_name}</div>
            <div class={"cpu-usage-frequency"}>{format!("{} MHz", props.cpu_stats.frequency)}</div>
            <div class={"cpu-usage-bar"}>
                <Meter
                    progress={progress as f64}
                />
            </div>
            <div class={"cpu-usage-usage"}>{format!("{:.2}%", props.cpu_stats.usage)}</div>
        </div>
    }
}
