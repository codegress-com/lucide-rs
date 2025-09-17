use sycamore::prelude::*;

#[derive(Props)]
pub struct TableColumnsSplitProps {
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
pub fn TableColumnsSplit(props: TableColumnsSplitProps) -> View {
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
            path(d="M14 14v2")
            path(d="M14 20v2")
            path(d="M14 2v2")
            path(d="M14 8v2")
            path(d="M2 15h8")
            path(d="M2 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H2")
            path(d="M2 9h8")
            path(d="M22 15h-4")
            path(d="M22 3h-2a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2")
            path(d="M22 9h-4")
            path(d="M5 3v18")
        }
    }
}
