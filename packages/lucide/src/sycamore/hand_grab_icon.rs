use sycamore::prelude::*;

#[derive(Props)]
pub struct HandGrabProps {
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
pub fn HandGrab(props: HandGrabProps) -> View {
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
            path(d="M18 11.5V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1.4")
            path(d="M14 10V8a2 2 0 0 0-2-2a2 2 0 0 0-2 2v2")
            path(d="M10 9.9V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v5")
            path(d="M6 14a2 2 0 0 0-2-2a2 2 0 0 0-2 2")
            path(d="M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0")
        }
    }
}
