use sycamore::prelude::*;

#[derive(Props)]
pub struct MonitorCogProps {
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
pub fn MonitorCog(props: MonitorCogProps) -> View {
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
            path(d="M12 17v4")
            path(d="m14.305 7.53.923-.382")
            path(d="m15.228 4.852-.923-.383")
            path(d="m16.852 3.228-.383-.924")
            path(d="m16.852 8.772-.383.923")
            path(d="m19.148 3.228.383-.924")
            path(d="m19.53 9.696-.382-.924")
            path(d="m20.772 4.852.924-.383")
            path(d="m20.772 7.148.924.383")
            path(d="M22 13v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7")
            path(d="M8 21h8")
            circle(cx="18", cy="6", r="3")
        }
    }
}
