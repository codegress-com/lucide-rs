use sycamore::prelude::*;

#[derive(Props)]
pub struct WaypointsProps {
    #[prop(default = 24)]
    pub size: usize,
    #[prop(default = String::from("currentColor"))]
    pub color: String,
    #[prop(default = String::from("none"))]
    pub fill: String,
    #[prop(default = 2)]
    pub stroke_width: usize,
    #[prop(default = false)]
    pub absolute_stroke_width: bool,
    #[prop(default)]
    pub class: Option<String>,
}

#[component]
pub fn Waypoints(props: WaypointsProps) -> View {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    let class = props.class.unwrap_or_default();
    
    view! {
        svg(
            xmlns="http://www.w3.org/2000/svg",
            class=format!("lucide {}", class),
            width=props.size.to_string(),
            height=props.size.to_string(),
            viewBox="0 0 24 24",
            fill=props.fill,
            stroke=props.color,
            "stroke-width"=stroke_width.to_string(),
            "stroke-linecap"="round",
            "stroke-linejoin"="round",
        ) {
            path(d="m10.2 6.3-3.9 3.9")
            path(d="M7 12h10")
            path(d="m13.8 17.7 3.9-3.9")
            circle(cx="12", cy="4.5", r="2.5")
            circle(cx="4.5", cy="12", r="2.5")
            circle(cx="19.5", cy="12", r="2.5")
            circle(cx="12", cy="19.5", r="2.5")
        }
    }
}
