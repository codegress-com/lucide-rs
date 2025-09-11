use sycamore::prelude::*;

#[derive(Props)]
pub struct RadioProps {
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
pub fn Radio(props: RadioProps) -> View {
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
            path(d="M16.247 7.761a6 6 0 0 1 0 8.478")
            path(d="M19.075 4.933a10 10 0 0 1 0 14.134")
            path(d="M4.925 19.067a10 10 0 0 1 0-14.134")
            path(d="M7.753 16.239a6 6 0 0 1 0-8.478")
            circle(cx="12", cy="12", r="2")
        }
    }
}
