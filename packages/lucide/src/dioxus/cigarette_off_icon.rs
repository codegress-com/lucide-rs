use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CigaretteOffProps {
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
pub fn CigaretteOff(props: CigaretteOffProps) -> Element {
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
            path { "d": "M12 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h13" }
            path { "d": "M18 8c0-2.5-2-2.5-2-5" }
            path { "d": "m2 2 20 20" }
            path { "d": "M21 12a1 1 0 0 1 1 1v2a1 1 0 0 1-.5.866" }
            path { "d": "M22 8c0-2.5-2-2.5-2-5" }
            path { "d": "M7 12v4" }
        }
    }
}
