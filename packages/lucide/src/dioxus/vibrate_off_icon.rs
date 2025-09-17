use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct VibrateOffProps {
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
pub fn VibrateOff(props: VibrateOffProps) -> Element {
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
            path { "d": "m2 8 2 2-2 2 2 2-2 2" }
            path { "d": "m22 8-2 2 2 2-2 2 2 2" }
            path { "d": "M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2" }
            path { "d": "M16 10.34V6c0-.55-.45-1-1-1h-4.34" }
            line { x1: "2", y1: "2", x2: "22", y2: "22" }
        }
    }
}
