use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct MeterProps {
    pub progress: f64,
}

#[function_component(Meter)]
pub fn meter(props: &MeterProps) -> Html {
    let progress = 100.0 * props.progress.clamp(0.0, 1.0);
    html! {
        <div class={"meter"}>
            <span style={format!("width: {progress}%")}></span>
        </div>
    }
}
