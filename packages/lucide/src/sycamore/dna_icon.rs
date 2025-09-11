use sycamore::prelude::*;

#[derive(Props)]
pub struct DnaProps {
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
pub fn Dna(props: DnaProps) -> View {
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
            path(d="m10 16 1.5 1.5")
            path(d="m14 8-1.5-1.5")
            path(d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993")
            path(d="m16.5 10.5 1 1")
            path(d="m17 6-2.891-2.891")
            path(d="M2 15c6.667-6 13.333 0 20-6")
            path(d="m20 9 .891.891")
            path(d="M3.109 14.109 4 15")
            path(d="m6.5 12.5 1 1")
            path(d="m7 18 2.891 2.891")
            path(d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993")
        }
    }
}
