use sycamore::prelude::*;

#[derive(Props)]
pub struct SmartphoneNfcProps {
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
pub fn SmartphoneNfc(props: SmartphoneNfcProps) -> View {
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
            path(d="M13 8.32a7.43 7.43 0 0 1 0 7.36")
            path(d="M16.46 6.21a11.76 11.76 0 0 1 0 11.58")
            path(d="M19.91 4.1a15.91 15.91 0 0 1 .01 15.8")
            rect(x="2", y="6", width="7", height="12")
        }
    }
}
