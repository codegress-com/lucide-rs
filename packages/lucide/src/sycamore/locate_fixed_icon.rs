use sycamore::prelude::*;

#[derive(Props)]
pub struct LocateFixedProps {
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
pub fn LocateFixed(props: LocateFixedProps) -> View {
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
            line(x1="2", y1="12", x2="5", y2="12")
            line(x1="19", y1="12", x2="22", y2="12")
            line(x1="12", y1="2", x2="12", y2="5")
            line(x1="12", y1="19", x2="12", y2="22")
            circle(cx="12", cy="12", r="7")
            circle(cx="12", cy="12", r="3")
        }
    }
}
