use sycamore::prelude::*;

#[derive(Props)]
pub struct WifiSyncProps {
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
pub fn WifiSync(props: WifiSyncProps) -> View {
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
            path(d="M11.965 10.105v4L13.5 12.5a5 5 0 0 1 8 1.5")
            path(d="M11.965 14.105h4")
            path(d="M17.965 18.105h4L20.43 19.71a5 5 0 0 1-8-1.5")
            path(d="M2 8.82a15 15 0 0 1 20 0")
            path(d="M21.965 22.105v-4")
            path(d="M5 12.86a10 10 0 0 1 3-2.032")
            path(d="M8.5 16.429h.01")
        }
    }
}
