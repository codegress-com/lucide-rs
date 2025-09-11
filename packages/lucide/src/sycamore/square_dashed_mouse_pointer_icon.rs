use sycamore::prelude::*;

#[derive(Props)]
pub struct SquareDashedMousePointerProps {
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
pub fn SquareDashedMousePointer(props: SquareDashedMousePointerProps) -> View {
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
            path(d="M12.034 12.681a.498.498 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.943l-3.444 1.068a1 1 0 0 0-.66.66l-1.067 3.443a.5.5 0 0 1-.943.033z")
            path(d="M5 3a2 2 0 0 0-2 2")
            path(d="M19 3a2 2 0 0 1 2 2")
            path(d="M5 21a2 2 0 0 1-2-2")
            path(d="M9 3h1")
            path(d="M9 21h2")
            path(d="M14 3h1")
            path(d="M3 9v1")
            path(d="M21 9v2")
            path(d="M3 14v1")
        }
    }
}
