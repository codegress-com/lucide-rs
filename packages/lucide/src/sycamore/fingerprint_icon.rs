use sycamore::prelude::*;

#[derive(Props)]
pub struct FingerprintProps {
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
pub fn Fingerprint(props: FingerprintProps) -> View {
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
            path(d="M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4")
            path(d="M14 13.12c0 2.38 0 6.38-1 8.88")
            path(d="M17.29 21.02c.12-.6.43-2.3.5-3.02")
            path(d="M2 12a10 10 0 0 1 18-6")
            path(d="M2 16h.01")
            path(d="M21.8 16c.2-2 .131-5.354 0-6")
            path(d="M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2")
            path(d="M8.65 22c.21-.66.45-1.32.57-2")
            path(d="M9 6.8a6 6 0 0 1 9 5.2v2")
        }
    }
}
