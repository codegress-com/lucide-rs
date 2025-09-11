use sycamore::prelude::*;

#[derive(Props)]
pub struct ScissorsLineDashedProps {
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
pub fn ScissorsLineDashed(props: ScissorsLineDashedProps) -> View {
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
            path(d="M5.42 9.42 8 12")
            path(d="m14 6-8.58 8.58")
            path(d="M10.8 14.8 14 18")
            path(d="M16 12h-2")
            path(d="M22 12h-2")
            circle(cx="4", cy="8", r="2")
            circle(cx="4", cy="16", r="2")
        }
    }
}
