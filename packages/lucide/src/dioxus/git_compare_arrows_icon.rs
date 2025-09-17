use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct GitCompareArrowsProps {
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
pub fn GitCompareArrows(props: GitCompareArrowsProps) -> Element {
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
            path { "d": "M12 6h5a2 2 0 0 1 2 2v7" }
            path { "d": "m15 9-3-3 3-3" }
            path { "d": "M12 18H7a2 2 0 0 1-2-2V9" }
            path { "d": "m9 15 3 3-3 3" }
            circle { cx: "5", cy: "6", r: "3" }
            circle { cx: "19", cy: "18", r: "3" }
        }
    }
}
