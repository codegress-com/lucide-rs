use sycamore::prelude::*;

#[derive(Props)]
pub struct CircleDotDashedProps {
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
pub fn CircleDotDashed(props: CircleDotDashedProps) -> View {
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
            path(d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0")
            path(d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7")
            path(d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8")
            path(d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69")
            path(d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0")
            path(d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7")
            path(d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8")
            path(d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69")
            circle(cx="12", cy="12", r="1")
        }
    }
}
