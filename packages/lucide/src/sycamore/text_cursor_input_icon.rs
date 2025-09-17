use sycamore::prelude::*;

#[derive(Props)]
pub struct TextCursorInputProps {
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
pub fn TextCursorInput(props: TextCursorInputProps) -> View {
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
            path(d="M12 20h-1a2 2 0 0 1-2-2 2 2 0 0 1-2 2H6")
            path(d="M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7")
            path(d="M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1")
            path(d="M6 4h1a2 2 0 0 1 2 2 2 2 0 0 1 2-2h1")
            path(d="M9 6v12")
        }
    }
}
