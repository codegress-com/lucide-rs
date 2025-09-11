use sycamore::prelude::*;

#[derive(Props)]
pub struct QrCodeProps {
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
pub fn QrCode(props: QrCodeProps) -> View {
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
            path(d="M21 16h-3a2 2 0 0 0-2 2v3")
            path(d="M21 21v.01")
            path(d="M12 7v3a2 2 0 0 1-2 2H7")
            path(d="M3 12h.01")
            path(d="M12 3h.01")
            path(d="M12 16v.01")
            path(d="M16 12h1")
            path(d="M21 12v.01")
            path(d="M12 21v-1")
            rect(x="3", y="3", width="5", height="5")
            rect(x="16", y="3", width="5", height="5")
            rect(x="3", y="16", width="5", height="5")
        }
    }
}
