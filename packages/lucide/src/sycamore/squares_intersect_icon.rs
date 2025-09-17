use sycamore::prelude::*;

#[derive(Props)]
pub struct SquaresIntersectProps {
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
pub fn SquaresIntersect(props: SquaresIntersectProps) -> View {
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
            path(d="M10 22a2 2 0 0 1-2-2")
            path(d="M14 2a2 2 0 0 1 2 2")
            path(d="M16 22h-2")
            path(d="M2 10V8")
            path(d="M2 4a2 2 0 0 1 2-2")
            path(d="M20 8a2 2 0 0 1 2 2")
            path(d="M22 14v2")
            path(d="M22 20a2 2 0 0 1-2 2")
            path(d="M4 16a2 2 0 0 1-2-2")
            path(d="M8 10a2 2 0 0 1 2-2h5a1 1 0 0 1 1 1v5a2 2 0 0 1-2 2H9a1 1 0 0 1-1-1z")
            path(d="M8 2h2")
        }
    }
}
