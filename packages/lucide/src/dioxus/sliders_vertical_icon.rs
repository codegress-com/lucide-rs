use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SlidersVerticalProps {
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
pub fn SlidersVertical(props: SlidersVerticalProps) -> Element {
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
            path { "d": "M10 8h4" }
            path { "d": "M12 21v-9" }
            path { "d": "M12 8V3" }
            path { "d": "M17 16h4" }
            path { "d": "M19 12V3" }
            path { "d": "M19 21v-5" }
            path { "d": "M3 14h4" }
            path { "d": "M5 10V3" }
            path { "d": "M5 21v-7" }
        }
    }
}
