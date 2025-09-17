use sycamore::prelude::*;

#[derive(Props)]
pub struct ScanBarcodeProps {
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
pub fn ScanBarcode(props: ScanBarcodeProps) -> View {
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
            path(d="M3 7V5a2 2 0 0 1 2-2h2")
            path(d="M17 3h2a2 2 0 0 1 2 2v2")
            path(d="M21 17v2a2 2 0 0 1-2 2h-2")
            path(d="M7 21H5a2 2 0 0 1-2-2v-2")
            path(d="M8 7v10")
            path(d="M12 7v10")
            path(d="M17 7v10")
        }
    }
}
