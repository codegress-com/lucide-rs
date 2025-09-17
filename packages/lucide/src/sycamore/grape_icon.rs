use sycamore::prelude::*;

#[derive(Props)]
pub struct GrapeProps {
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
pub fn Grape(props: GrapeProps) -> View {
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
            path(d="M22 5V2l-5.89 5.89")
            circle(cx="16.6", cy="15.89", r="3")
            circle(cx="8.11", cy="7.4", r="3")
            circle(cx="12.35", cy="11.65", r="3")
            circle(cx="13.91", cy="5.85", r="3")
            circle(cx="18.15", cy="10.09", r="3")
            circle(cx="6.56", cy="13.2", r="3")
            circle(cx="10.8", cy="17.44", r="3")
            circle(cx="5", cy="19", r="3")
        }
    }
}
