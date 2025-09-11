use sycamore::prelude::*;

#[derive(Props)]
pub struct LocateOffProps {
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
pub fn LocateOff(props: LocateOffProps) -> View {
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
            path(d="M12 19v3")
            path(d="M12 2v3")
            path(d="M18.89 13.24a7 7 0 0 0-8.13-8.13")
            path(d="M19 12h3")
            path(d="M2 12h3")
            path(d="m2 2 20 20")
            path(d="M7.05 7.05a7 7 0 0 0 9.9 9.9")
        }
    }
}
