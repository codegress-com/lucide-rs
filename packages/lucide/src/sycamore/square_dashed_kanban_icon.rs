use sycamore::prelude::*;

#[derive(Props)]
pub struct SquareDashedKanbanProps {
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
pub fn SquareDashedKanban(props: SquareDashedKanbanProps) -> View {
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
            path(d="M8 7v7")
            path(d="M12 7v4")
            path(d="M16 7v9")
            path(d="M5 3a2 2 0 0 0-2 2")
            path(d="M9 3h1")
            path(d="M14 3h1")
            path(d="M19 3a2 2 0 0 1 2 2")
            path(d="M21 9v1")
            path(d="M21 14v1")
            path(d="M21 19a2 2 0 0 1-2 2")
            path(d="M14 21h1")
            path(d="M9 21h1")
            path(d="M5 21a2 2 0 0 1-2-2")
            path(d="M3 14v1")
            path(d="M3 9v1")
        }
    }
}
