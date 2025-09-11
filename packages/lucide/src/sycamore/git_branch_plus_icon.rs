use sycamore::prelude::*;

#[derive(Props)]
pub struct GitBranchPlusProps {
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
pub fn GitBranchPlus(props: GitBranchPlusProps) -> View {
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
            path(d="M6 3v12")
            path(d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z")
            path(d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z")
            path(d="M15 6a9 9 0 0 0-9 9")
            path(d="M18 15v6")
            path(d="M21 18h-6")
        }
    }
}
