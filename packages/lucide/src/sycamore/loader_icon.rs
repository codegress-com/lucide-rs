use sycamore::prelude::*;

#[derive(Props)]
pub struct LoaderProps {
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
pub fn Loader(props: LoaderProps) -> View {
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
            path(d="M12 2v4")
            path(d="m16.2 7.8 2.9-2.9")
            path(d="M18 12h4")
            path(d="m16.2 16.2 2.9 2.9")
            path(d="M12 18v4")
            path(d="m4.9 19.1 2.9-2.9")
            path(d="M2 12h4")
            path(d="m4.9 4.9 2.9 2.9")
        }
    }
}
