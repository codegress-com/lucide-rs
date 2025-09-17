use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct GpuProps {
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
pub fn Gpu(props: GpuProps) -> Element {
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
            path { "d": "M2 21V3" }
            path { "d": "M2 5h18a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2.26" }
            path { "d": "M7 17v3a1 1 0 0 0 1 1h5a1 1 0 0 0 1-1v-3" }
            circle { cx: "16", cy: "11", r: "2" }
            circle { cx: "8", cy: "11", r: "2" }
        }
    }
}
