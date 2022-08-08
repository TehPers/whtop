use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct ProgressRingProps {
    #[prop_or_else(|| 60.0)]
    pub radius: f64,
    #[prop_or_else(|| 1.0)]
    pub stroke: f64,
    #[prop_or_else(|| "var(--theme-primary)".into())]
    pub stroke_color: AttrValue,
    pub progress: f64,
}

#[function_component(ProgressRing)]
pub fn progress_ring(props: &ProgressRingProps) -> Html {
    let inner_radius = props.radius - props.stroke * 2.0;
    let circumference = inner_radius * 2.0 * std::f64::consts::PI;
    let offset = circumference - props.progress * circumference;
    html! {
        <svg
            width={(props.radius * 2.0).to_string()}
            height={(props.radius * 2.0).to_string()}
            class={"progress-ring"}
        >
            <circle
                stroke={props.stroke_color.clone()}
                stroke-dasharray={format!("{circumference} {circumference}")}
                style={format!("stroke-dashoffset: {offset}")}
                stroke-width={props.stroke.to_string()}
                fill={"transparent"}
                r={inner_radius.to_string()}
                cx={props.radius.to_string()}
                cy={props.radius.to_string()}
            />
        </svg>
    }
}
