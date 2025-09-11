use sycamore::prelude::*;

#[derive(Props)]
pub struct VectorSquareProps {
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
pub fn VectorSquare(props: VectorSquareProps) -> View {
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
            path(d="M19.5 7a24 24 0 0 1 0 10")
            path(d="M4.5 7a24 24 0 0 0 0 10")
            path(d="M7 19.5a24 24 0 0 0 10 0")
            path(d="M7 4.5a24 24 0 0 1 10 0")
            rect(x="17", y="17", width="5", height="5")
            rect(x="17", y="2", width="5", height="5")
            rect(x="2", y="17", width="5", height="5")
            rect(x="2", y="2", width="5", height="5")
        }
    }
}
