use sycamore::prelude::*;

#[derive(Props)]
pub struct LoaderPinwheelProps {
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
pub fn LoaderPinwheel(props: LoaderPinwheelProps) -> View {
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
            path(d="M22 12a1 1 0 0 1-10 0 1 1 0 0 0-10 0")
            path(d="M7 20.7a1 1 0 1 1 5-8.7 1 1 0 1 0 5-8.6")
            path(d="M7 3.3a1 1 0 1 1 5 8.6 1 1 0 1 0 5 8.6")
            circle(cx="12", cy="12", r="10")
        }
    }
}
