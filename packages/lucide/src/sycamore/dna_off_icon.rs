use sycamore::prelude::*;

#[derive(Props)]
pub struct DnaOffProps {
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
pub fn DnaOff(props: DnaOffProps) -> View {
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
            path(d="M15 2c-1.35 1.5-2.092 3-2.5 4.5L14 8")
            path(d="m17 6-2.891-2.891")
            path(d="M2 15c3.333-3 6.667-3 10-3")
            path(d="m2 2 20 20")
            path(d="m20 9 .891.891")
            path(d="M22 9c-1.5 1.35-3 2.092-4.5 2.5l-1-1")
            path(d="M3.109 14.109 4 15")
            path(d="m6.5 12.5 1 1")
            path(d="m7 18 2.891 2.891")
            path(d="M9 22c1.35-1.5 2.092-3 2.5-4.5L10 16")
        }
    }
}
