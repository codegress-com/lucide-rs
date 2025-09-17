use sycamore::prelude::*;

#[derive(Props)]
pub struct SquircleDashedProps {
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
pub fn SquircleDashed(props: SquircleDashedProps) -> View {
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
            path(d="M13.77 3.043a34 34 0 0 0-3.54 0")
            path(d="M13.771 20.956a33 33 0 0 1-3.541.001")
            path(d="M20.18 17.74c-.51 1.15-1.29 1.93-2.439 2.44")
            path(d="M20.18 6.259c-.51-1.148-1.291-1.929-2.44-2.438")
            path(d="M20.957 10.23a33 33 0 0 1 0 3.54")
            path(d="M3.043 10.23a34 34 0 0 0 .001 3.541")
            path(d="M6.26 20.179c-1.15-.508-1.93-1.29-2.44-2.438")
            path(d="M6.26 3.82c-1.149.51-1.93 1.291-2.44 2.44")
        }
    }
}
