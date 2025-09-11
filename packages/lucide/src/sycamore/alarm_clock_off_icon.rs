use sycamore::prelude::*;

#[derive(Props)]
pub struct AlarmClockOffProps {
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
pub fn AlarmClockOff(props: AlarmClockOffProps) -> View {
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
            path(d="M6.87 6.87a8 8 0 1 0 11.26 11.26")
            path(d="M19.9 14.25a8 8 0 0 0-9.15-9.15")
            path(d="m22 6-3-3")
            path(d="M6.26 18.67 4 21")
            path(d="m2 2 20 20")
            path(d="M4 4 2 6")
        }
    }
}
