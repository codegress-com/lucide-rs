use sycamore::prelude::*;

#[derive(Props)]
pub struct ImageUpscaleProps {
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
pub fn ImageUpscale(props: ImageUpscaleProps) -> View {
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
            path(d="M16 3h5v5")
            path(d="M17 21h2a2 2 0 0 0 2-2")
            path(d="M21 12v3")
            path(d="m21 3-5 5")
            path(d="M3 7V5a2 2 0 0 1 2-2")
            path(d="m5 21 4.144-4.144a1.21 1.21 0 0 1 1.712 0L13 19")
            path(d="M9 3h3")
            rect(x="3", y="11", width="10", height="10")
        }
    }
}
