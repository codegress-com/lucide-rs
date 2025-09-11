use sycamore::prelude::*;

#[derive(Props)]
pub struct CastleProps {
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
pub fn Castle(props: CastleProps) -> View {
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
            path(d="M10 5V3")
            path(d="M14 5V3")
            path(d="M15 21v-3a3 3 0 0 0-6 0v3")
            path(d="M18 3v8")
            path(d="M18 5H6")
            path(d="M22 11H2")
            path(d="M22 9v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9")
            path(d="M6 3v8")
        }
    }
}
