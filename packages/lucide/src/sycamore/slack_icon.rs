use sycamore::prelude::*;

#[derive(Props)]
pub struct SlackProps {
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
pub fn Slack(props: SlackProps) -> View {
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
            path(d="M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5")
            path(d="M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5")
            path(d="M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5")
            path(d="M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5")
            rect(x="13", y="2", width="3", height="8")
            rect(x="8", y="14", width="3", height="8")
            rect(x="14", y="13", width="8", height="3")
            rect(x="2", y="8", width="8", height="3")
        }
    }
}
