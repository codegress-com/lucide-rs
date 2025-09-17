use sycamore::prelude::*;

#[derive(Props)]
pub struct SandwichProps {
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
pub fn Sandwich(props: SandwichProps) -> View {
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
            path(d="m2.37 11.223 8.372-6.777a2 2 0 0 1 2.516 0l8.371 6.777")
            path(d="M21 15a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-5.25")
            path(d="M3 15a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h9")
            path(d="m6.67 15 6.13 4.6a2 2 0 0 0 2.8-.4l3.15-4.2")
            rect(x="2", y="11", width="20", height="4")
        }
    }
}
