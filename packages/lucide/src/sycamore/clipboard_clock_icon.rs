use sycamore::prelude::*;

#[derive(Props)]
pub struct ClipboardClockProps {
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
pub fn ClipboardClock(props: ClipboardClockProps) -> View {
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
            path(d="M16 14v2.2l1.6 1")
            path(d="M16 4h2a2 2 0 0 1 2 2v.832")
            path(d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2")
            circle(cx="16", cy="16", r="6")
            rect(x="8", y="2", width="8", height="4")
        }
    }
}
