use sycamore::prelude::*;

#[derive(Props)]
pub struct BaggageClaimProps {
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
pub fn BaggageClaim(props: BaggageClaimProps) -> View {
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
            path(d="M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2")
            path(d="M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10")
            rect(x="8", y="6", width="13", height="8")
            circle(cx="18", cy="20", r="2")
            circle(cx="9", cy="20", r="2")
        }
    }
}
