use sycamore::prelude::*;

#[derive(Props)]
pub struct ShowerHeadProps {
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
pub fn ShowerHead(props: ShowerHeadProps) -> View {
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
            path(d="m4 4 2.5 2.5")
            path(d="M13.5 6.5a4.95 4.95 0 0 0-7 7")
            path(d="M15 5 5 15")
            path(d="M14 17v.01")
            path(d="M10 16v.01")
            path(d="M13 13v.01")
            path(d="M16 10v.01")
            path(d="M11 20v.01")
            path(d="M17 14v.01")
            path(d="M20 11v.01")
        }
    }
}
