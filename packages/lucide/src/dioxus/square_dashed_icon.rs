use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SquareDashedProps {
    #[props(default = 24)]
    pub size: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    #[props(default = "none".to_owned())]
    pub fill: String,
    #[props(default = 2)]
    pub stroke_width: usize,
    #[props(default = false)]
    pub absolute_stroke_width: bool,
    pub class: Option<String>,
    pub style: Option<String>,
}

#[component]
pub fn SquareDashed(props: SquareDashedProps) -> Element {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class { "{class}" },
            "style": if let Some(style) = props.style { "{style}" },
            "width": "{props.size}",
            "height": "{props.size}",
            "viewBox": "0 0 24 24",
            "fill": "{props.fill}",
            "stroke": "{props.color}",
            "stroke-width": "{stroke_width}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { "d": "M5 3a2 2 0 0 0-2 2" }
            path { "d": "M19 3a2 2 0 0 1 2 2" }
            path { "d": "M21 19a2 2 0 0 1-2 2" }
            path { "d": "M5 21a2 2 0 0 1-2-2" }
            path { "d": "M9 3h1" }
            path { "d": "M9 21h1" }
            path { "d": "M14 3h1" }
            path { "d": "M14 21h1" }
            path { "d": "M3 9v1" }
            path { "d": "M21 9v1" }
            path { "d": "M3 14v1" }
            path { "d": "M21 14v1" }
        }
    }
}
