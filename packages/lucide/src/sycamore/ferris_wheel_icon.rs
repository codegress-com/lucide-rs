use sycamore::prelude::*;

#[derive(Props)]
pub struct FerrisWheelProps {
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
pub fn FerrisWheel(props: FerrisWheelProps) -> View {
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
            path(d="M12 2v4")
            path(d="m6.8 15-3.5 2")
            path(d="m20.7 7-3.5 2")
            path(d="M6.8 9 3.3 7")
            path(d="m20.7 17-3.5-2")
            path(d="m9 22 3-8 3 8")
            path(d="M8 22h8")
            path(d="M18 18.7a9 9 0 1 0-12 0")
            circle(cx="12", cy="12", r="2")
        }
    }
}
