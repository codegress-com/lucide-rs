use sycamore::prelude::*;

#[derive(Props)]
pub struct Building2Props {
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
pub fn Building2(props: Building2Props) -> View {
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
            path(d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z")
            path(d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2")
            path(d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2")
            path(d="M10 6h4")
            path(d="M10 10h4")
            path(d="M10 14h4")
            path(d="M10 18h4")
        }
    }
}
