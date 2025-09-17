use sycamore::prelude::*;

#[derive(Props)]
pub struct ChartScatterProps {
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
pub fn ChartScatter(props: ChartScatterProps) -> View {
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
            path(d="M3 3v16a2 2 0 0 0 2 2h16")
            circle(cx="7.5", cy="7.5", r=".5")
            circle(cx="18.5", cy="5.5", r=".5")
            circle(cx="11.5", cy="11.5", r=".5")
            circle(cx="7.5", cy="16.5", r=".5")
            circle(cx="17.5", cy="14.5", r=".5")
        }
    }
}
