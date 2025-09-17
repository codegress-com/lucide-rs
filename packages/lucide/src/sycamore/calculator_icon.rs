use sycamore::prelude::*;

#[derive(Props)]
pub struct CalculatorProps {
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
pub fn Calculator(props: CalculatorProps) -> View {
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
            path(d="M16 10h.01")
            path(d="M12 10h.01")
            path(d="M8 10h.01")
            path(d="M12 14h.01")
            path(d="M8 14h.01")
            path(d="M12 18h.01")
            path(d="M8 18h.01")
            rect(x="4", y="2", width="16", height="20")
            line(x1="8", y1="6", x2="16", y2="6")
            line(x1="16", y1="14", x2="16", y2="18")
        }
    }
}
