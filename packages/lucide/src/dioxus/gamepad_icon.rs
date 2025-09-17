use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct GamepadProps {
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
pub fn Gamepad(props: GamepadProps) -> Element {
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
            line { x1: "6", y1: "12", x2: "10", y2: "12" }
            line { x1: "8", y1: "10", x2: "8", y2: "14" }
            line { x1: "15", y1: "13", x2: "15.01", y2: "13" }
            line { x1: "18", y1: "11", x2: "18.01", y2: "11" }
            rect { x: "2", y: "6", width: "20", height: "12" }
        }
    }
}
