use sycamore::prelude::*;

#[derive(Props)]
pub struct ThermometerSnowflakeProps {
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
pub fn ThermometerSnowflake(props: ThermometerSnowflakeProps) -> View {
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
            path(d="m10 20-1.25-2.5L6 18")
            path(d="M10 4 8.75 6.5 6 6")
            path(d="M10.585 15H10")
            path(d="M2 12h6.5L10 9")
            path(d="M20 14.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0z")
            path(d="m4 10 1.5 2L4 14")
            path(d="m7 21 3-6-1.5-3")
            path(d="m7 3 3 6h2")
        }
    }
}
