use sycamore::prelude::*;

#[derive(Props)]
pub struct SunsetProps {
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
pub fn Sunset(props: SunsetProps) -> View {
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
            path(d="M12 10V2")
            path(d="m4.93 10.93 1.41 1.41")
            path(d="M2 18h2")
            path(d="M20 18h2")
            path(d="m19.07 10.93-1.41 1.41")
            path(d="M22 22H2")
            path(d="m16 6-4 4-4-4")
            path(d="M16 18a4 4 0 0 0-8 0")
        }
    }
}
