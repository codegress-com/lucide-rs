use sycamore::prelude::*;

#[derive(Props)]
pub struct MessageCircleDashedProps {
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
pub fn MessageCircleDashed(props: MessageCircleDashedProps) -> View {
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
            path(d="M10.1 2.182a10 10 0 0 1 3.8 0")
            path(d="M13.9 21.818a10 10 0 0 1-3.8 0")
            path(d="M17.609 3.72a10 10 0 0 1 2.69 2.7")
            path(d="M2.182 13.9a10 10 0 0 1 0-3.8")
            path(d="M20.28 17.61a10 10 0 0 1-2.7 2.69")
            path(d="M21.818 10.1a10 10 0 0 1 0 3.8")
            path(d="M3.721 6.391a10 10 0 0 1 2.7-2.69")
            path(d="m6.163 21.117-2.906.85a1 1 0 0 1-1.236-1.169l.965-2.98")
        }
    }
}
