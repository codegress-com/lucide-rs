use sycamore::prelude::*;

#[derive(Props)]
pub struct AlignCenterHorizontalProps {
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
pub fn AlignCenterHorizontal(props: AlignCenterHorizontalProps) -> View {
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
            path(d="M2 12h20")
            path(d="M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4")
            path(d="M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4")
            path(d="M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1")
            path(d="M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1")
        }
    }
}
