use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct Share2Props {
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
pub fn Share2(props: Share2Props) -> Element {
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
            circle { cx: "18", cy: "5", r: "3" }
            circle { cx: "6", cy: "12", r: "3" }
            circle { cx: "18", cy: "19", r: "3" }
            line { x1: "8.59", y1: "13.51", x2: "15.42", y2: "17.49" }
            line { x1: "15.41", y1: "6.51", x2: "8.59", y2: "10.49" }
        }
    }
}
