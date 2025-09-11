use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct GitBranchPlusProps {
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
pub fn GitBranchPlus(props: GitBranchPlusProps) -> Element {
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
            path { "d": "M6 3v12" }
            path { "d": "M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" }
            path { "d": "M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" }
            path { "d": "M15 6a9 9 0 0 0-9 9" }
            path { "d": "M18 15v6" }
            path { "d": "M21 18h-6" }
        }
    }
}
