use sycamore::prelude::*;

#[derive(Props)]
pub struct CpuProps {
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
pub fn Cpu(props: CpuProps) -> View {
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
            path(d="M12 20v2")
            path(d="M12 2v2")
            path(d="M17 20v2")
            path(d="M17 2v2")
            path(d="M2 12h2")
            path(d="M2 17h2")
            path(d="M2 7h2")
            path(d="M20 12h2")
            path(d="M20 17h2")
            path(d="M20 7h2")
            path(d="M7 20v2")
            path(d="M7 2v2")
            rect(x="4", y="4", width="16", height="16")
            rect(x="8", y="8", width="8", height="8")
        }
    }
}
