use sycamore::prelude::*;

#[derive(Props)]
pub struct GpuProps {
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
pub fn Gpu(props: GpuProps) -> View {
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
            path(d="M2 21V3")
            path(d="M2 5h18a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2.26")
            path(d="M7 17v3a1 1 0 0 0 1 1h5a1 1 0 0 0 1-1v-3")
            circle(cx="16", cy="11", r="2")
            circle(cx="8", cy="11", r="2")
        }
    }
}
