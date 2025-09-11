use sycamore::prelude::*;

#[derive(Props)]
pub struct ScaleProps {
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
pub fn Scale(props: ScaleProps) -> View {
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
            path(d="m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z")
            path(d="m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z")
            path(d="M7 21h10")
            path(d="M12 3v18")
            path(d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2")
        }
    }
}
