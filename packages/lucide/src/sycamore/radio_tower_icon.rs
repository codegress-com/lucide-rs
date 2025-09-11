use sycamore::prelude::*;

#[derive(Props)]
pub struct RadioTowerProps {
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
pub fn RadioTower(props: RadioTowerProps) -> View {
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
            path(d="M4.9 16.1C1 12.2 1 5.8 4.9 1.9")
            path(d="M7.8 4.7a6.14 6.14 0 0 0-.8 7.5")
            path(d="M16.2 4.8c2 2 2.26 5.11.8 7.47")
            path(d="M19.1 1.9a9.96 9.96 0 0 1 0 14.1")
            path(d="M9.5 18h5")
            path(d="m8 22 4-11 4 11")
            circle(cx="12", cy="9", r="2")
        }
    }
}
