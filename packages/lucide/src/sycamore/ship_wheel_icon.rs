use sycamore::prelude::*;

#[derive(Props)]
pub struct ShipWheelProps {
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
pub fn ShipWheel(props: ShipWheelProps) -> View {
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
            path(d="M12 2v7.5")
            path(d="m19 5-5.23 5.23")
            path(d="M22 12h-7.5")
            path(d="m19 19-5.23-5.23")
            path(d="M12 14.5V22")
            path(d="M10.23 13.77 5 19")
            path(d="M9.5 12H2")
            path(d="M10.23 10.23 5 5")
            circle(cx="12", cy="12", r="8")
            circle(cx="12", cy="12", r="2.5")
        }
    }
}
