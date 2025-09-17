use sycamore::prelude::*;

#[derive(Props)]
pub struct TractorProps {
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
pub fn Tractor(props: TractorProps) -> View {
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
            path(d="m10 11 11 .9a1 1 0 0 1 .8 1.1l-.665 4.158a1 1 0 0 1-.988.842H20")
            path(d="M16 18h-5")
            path(d="M18 5a1 1 0 0 0-1 1v5.573")
            path(d="M3 4h8.129a1 1 0 0 1 .99.863L13 11.246")
            path(d="M4 11V4")
            path(d="M7 15h.01")
            path(d="M8 10.1V4")
            circle(cx="18", cy="18", r="2")
            circle(cx="7", cy="15", r="5")
        }
    }
}
