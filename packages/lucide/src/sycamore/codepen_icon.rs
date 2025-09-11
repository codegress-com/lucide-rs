use sycamore::prelude::*;

#[derive(Props)]
pub struct CodepenProps {
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
pub fn Codepen(props: CodepenProps) -> View {
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
            polygon(points="12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2")
            line(x1="12", y1="22", x2="12", y2="15.5")
            polyline(points="22 8.5 12 15.5 2 8.5")
            polyline(points="2 15.5 12 8.5 22 15.5")
            line(x1="12", y1="2", x2="12", y2="8.5")
        }
    }
}
