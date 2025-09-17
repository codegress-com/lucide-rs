use sycamore::prelude::*;

#[derive(Props)]
pub struct ServerCogProps {
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
pub fn ServerCog(props: ServerCogProps) -> View {
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
            path(d="m10.852 14.772-.383.923")
            path(d="M13.148 14.772a3 3 0 1 0-2.296-5.544l-.383-.923")
            path(d="m13.148 9.228.383-.923")
            path(d="m13.53 15.696-.382-.924a3 3 0 1 1-2.296-5.544")
            path(d="m14.772 10.852.923-.383")
            path(d="m14.772 13.148.923.383")
            path(d="M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5")
            path(d="M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5")
            path(d="M6 18h.01")
            path(d="M6 6h.01")
            path(d="m9.228 10.852-.923-.383")
            path(d="m9.228 13.148-.923.383")
        }
    }
}
