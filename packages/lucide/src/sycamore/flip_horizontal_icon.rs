use sycamore::prelude::*;

#[derive(Props)]
pub struct FlipHorizontalProps {
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
pub fn FlipHorizontal(props: FlipHorizontalProps) -> View {
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
            path(d="M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3")
            path(d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3")
            path(d="M12 20v2")
            path(d="M12 14v2")
            path(d="M12 8v2")
            path(d="M12 2v2")
        }
    }
}
