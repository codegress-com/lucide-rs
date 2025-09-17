use sycamore::prelude::*;

#[derive(Props)]
pub struct SmilePlusProps {
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
pub fn SmilePlus(props: SmilePlusProps) -> View {
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
            path(d="M22 11v1a10 10 0 1 1-9-10")
            path(d="M8 14s1.5 2 4 2 4-2 4-2")
            path(d="M16 5h6")
            path(d="M19 2v6")
            line(x1="9", y1="9", x2="9.01", y2="9")
            line(x1="15", y1="9", x2="15.01", y2="9")
        }
    }
}
