use sycamore::prelude::*;

#[derive(Props)]
pub struct WifiOffProps {
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
pub fn WifiOff(props: WifiOffProps) -> View {
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
            path(d="M12 20h.01")
            path(d="M8.5 16.429a5 5 0 0 1 7 0")
            path(d="M5 12.859a10 10 0 0 1 5.17-2.69")
            path(d="M19 12.859a10 10 0 0 0-2.007-1.523")
            path(d="M2 8.82a15 15 0 0 1 4.177-2.643")
            path(d="M22 8.82a15 15 0 0 0-11.288-3.764")
            path(d="m2 2 20 20")
        }
    }
}
