use sycamore::prelude::*;

#[derive(Props)]
pub struct SunSnowProps {
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
pub fn SunSnow(props: SunSnowProps) -> View {
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
            path(d="M10 21v-1")
            path(d="M10 4V3")
            path(d="M10 9a3 3 0 0 0 0 6")
            path(d="m14 20 1.25-2.5L18 18")
            path(d="m14 4 1.25 2.5L18 6")
            path(d="m17 21-3-6 1.5-3H22")
            path(d="m17 3-3 6 1.5 3")
            path(d="M2 12h1")
            path(d="m20 10-1.5 2 1.5 2")
            path(d="m3.64 18.36.7-.7")
            path(d="m4.34 6.34-.7-.7")
        }
    }
}
