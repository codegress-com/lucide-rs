use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BoomBoxProps {
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
pub fn BoomBox(props: BoomBoxProps) -> Element {
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
            path { "d": "M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" }
            path { "d": "M8 8v1" }
            path { "d": "M12 8v1" }
            path { "d": "M16 8v1" }
            rect { x: "2", y: "9", width: "20", height: "12" }
            circle { cx: "8", cy: "15", r: "2" }
            circle { cx: "16", cy: "15", r: "2" }
        }
    }
}
