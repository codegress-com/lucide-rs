use sycamore::prelude::*;

#[derive(Props)]
pub struct HotelProps {
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
pub fn Hotel(props: HotelProps) -> View {
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
            path(d="M10 22v-6.57")
            path(d="M12 11h.01")
            path(d="M12 7h.01")
            path(d="M14 15.43V22")
            path(d="M15 16a5 5 0 0 0-6 0")
            path(d="M16 11h.01")
            path(d="M16 7h.01")
            path(d="M8 11h.01")
            path(d="M8 7h.01")
            rect(x="4", y="2", width="16", height="20")
        }
    }
}
