use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct LocateProps {
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
pub fn Locate(props: LocateProps) -> Element {
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
            line { x1: "2", y1: "12", x2: "5", y2: "12" }
            line { x1: "19", y1: "12", x2: "22", y2: "12" }
            line { x1: "12", y1: "2", x2: "12", y2: "5" }
            line { x1: "12", y1: "19", x2: "12", y2: "22" }
            circle { cx: "12", cy: "12", r: "7" }
        }
    }
}
