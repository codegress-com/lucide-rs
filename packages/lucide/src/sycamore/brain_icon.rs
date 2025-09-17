use sycamore::prelude::*;

#[derive(Props)]
pub struct BrainProps {
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
pub fn Brain(props: BrainProps) -> View {
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
            path(d="M12 18V5")
            path(d="M15 13a4.17 4.17 0 0 1-3-4 4.17 4.17 0 0 1-3 4")
            path(d="M17.598 6.5A3 3 0 1 0 12 5a3 3 0 1 0-5.598 1.5")
            path(d="M17.997 5.125a4 4 0 0 1 2.526 5.77")
            path(d="M18 18a4 4 0 0 0 2-7.464")
            path(d="M19.967 17.483A4 4 0 1 1 12 18a4 4 0 1 1-7.967-.517")
            path(d="M6 18a4 4 0 0 1-2-7.464")
            path(d="M6.003 5.125a4 4 0 0 0-2.526 5.77")
        }
    }
}
