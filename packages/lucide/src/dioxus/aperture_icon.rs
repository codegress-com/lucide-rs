use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ApertureProps {
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
pub fn Aperture(props: ApertureProps) -> Element {
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
            path { "d": "m14.31 8 5.74 9.94" }
            path { "d": "M9.69 8h11.48" }
            path { "d": "m7.38 12 5.74-9.94" }
            path { "d": "M9.69 16 3.95 6.06" }
            path { "d": "M14.31 16H2.83" }
            path { "d": "m16.62 12-5.74 9.94" }
            circle { cx: "12", cy: "12", r: "10" }
        }
    }
}
