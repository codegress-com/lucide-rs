use sycamore::prelude::*;

#[derive(Props)]
pub struct ThermometerSunProps {
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
pub fn ThermometerSun(props: ThermometerSunProps) -> View {
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
            path(d="M12 9a4 4 0 0 0-2 7.5")
            path(d="M12 3v2")
            path(d="m6.6 18.4-1.4 1.4")
            path(d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z")
            path(d="M4 13H2")
            path(d="M6.34 7.34 4.93 5.93")
        }
    }
}
