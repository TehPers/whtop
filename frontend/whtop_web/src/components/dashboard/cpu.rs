use crate::components::Meter;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CpuUsageProps {
    #[prop_or_default]
    pub cpu_name: Option<String>,
    #[prop_or_default]
    pub cpu_frequency: Option<u64>,
    pub cpu_usage: f32,
}

#[function_component(CpuUsage)]
pub fn cpu_usage(props: &CpuUsageProps) -> Html {
    let progress = props.cpu_usage / 100.0;
    html! {
        <div class={"cpu-usage"}>
            {
                if let Some(cpu_name) = props.cpu_name.as_ref() {
                    html! {
                        <div class={"cpu-usage-name"}>{cpu_name}</div>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(cpu_frequency) = props.cpu_frequency {
                    html! {
                        <div class={"cpu-usage-frequency"}>{format!("{} MHz", cpu_frequency)}</div>
                    }
                } else {
                    html! {}
                }
            }
            <div class={"cpu-usage-bar"}>
                <Meter
                    progress={progress as f64}
                />
            </div>
            <div class={"cpu-usage-usage"}>{format!("{:.2}%", props.cpu_usage)}</div>
        </div>
    }
}
