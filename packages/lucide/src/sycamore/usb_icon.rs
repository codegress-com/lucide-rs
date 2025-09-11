use sycamore::prelude::*;

#[derive(Props)]
pub struct UsbProps {
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
pub fn Usb(props: UsbProps) -> View {
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
            path(d="M4.7 19.3 19 5")
            path(d="m21 3-3 1 2 2Z")
            path(d="M9.26 7.68 5 12l2 5")
            path(d="m10 14 5 2 3.5-3.5")
            path(d="m18 12 1-1 1 1-1 1Z")
            circle(cx="10", cy="7", r="1")
            circle(cx="4", cy="20", r="1")
        }
    }
}
