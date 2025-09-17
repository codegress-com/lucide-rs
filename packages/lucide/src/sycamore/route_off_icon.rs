use sycamore::prelude::*;

#[derive(Props)]
pub struct RouteOffProps {
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
pub fn RouteOff(props: RouteOffProps) -> View {
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
            path(d="M9 19h8.5c.4 0 .9-.1 1.3-.2")
            path(d="M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12")
            path(d="m2 2 20 20")
            path(d="M21 15.3a3.5 3.5 0 0 0-3.3-3.3")
            path(d="M15 5h-4.3")
            circle(cx="6", cy="19", r="3")
            circle(cx="18", cy="5", r="3")
        }
    }
}
