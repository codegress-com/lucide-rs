use sycamore::prelude::*;

#[derive(Props)]
pub struct TrophyProps {
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
pub fn Trophy(props: TrophyProps) -> View {
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
            path(d="M10 14.66v1.626a2 2 0 0 1-.976 1.696A5 5 0 0 0 7 21.978")
            path(d="M14 14.66v1.626a2 2 0 0 0 .976 1.696A5 5 0 0 1 17 21.978")
            path(d="M18 9h1.5a1 1 0 0 0 0-5H18")
            path(d="M4 22h16")
            path(d="M6 9a6 6 0 0 0 12 0V3a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1z")
            path(d="M6 9H4.5a1 1 0 0 1 0-5H6")
        }
    }
}
