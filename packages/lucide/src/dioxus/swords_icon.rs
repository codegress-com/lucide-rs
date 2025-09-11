use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SwordsProps {
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
pub fn Swords(props: SwordsProps) -> Element {
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
            polyline { points: "14.5 17.5 3 6 3 3 6 3 17.5 14.5" }
            line { x1: "13", y1: "19", x2: "19", y2: "13" }
            line { x1: "16", y1: "16", x2: "20", y2: "20" }
            line { x1: "19", y1: "21", x2: "21", y2: "19" }
            polyline { points: "14.5 6.5 18 3 21 3 21 6 17.5 9.5" }
            line { x1: "5", y1: "14", x2: "9", y2: "18" }
            line { x1: "7", y1: "17", x2: "4", y2: "20" }
            line { x1: "3", y1: "19", x2: "5", y2: "21" }
        }
    }
}
