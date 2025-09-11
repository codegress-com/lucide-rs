use sycamore::prelude::*;

#[derive(Props)]
pub struct TableRowsSplitProps {
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
pub fn TableRowsSplit(props: TableRowsSplitProps) -> View {
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
            path(d="M14 10h2")
            path(d="M15 22v-8")
            path(d="M15 2v4")
            path(d="M2 10h2")
            path(d="M20 10h2")
            path(d="M3 19h18")
            path(d="M3 22v-6a2 2 135 0 1 2-2h14a2 2 45 0 1 2 2v6")
            path(d="M3 2v2a2 2 45 0 0 2 2h14a2 2 135 0 0 2-2V2")
            path(d="M8 10h2")
            path(d="M9 22v-8")
            path(d="M9 2v4")
        }
    }
}
