use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ALargeSmallProps {
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
pub fn ALargeSmall(props: ALargeSmallProps) -> Element {
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
            path { "d": "m15 16 2.536-7.328a1.02 1.02 1 0 1 1.928 0L22 16" }
            path { "d": "M15.697 14h5.606" }
            path { "d": "m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16" }
            path { "d": "M3.304 13h6.392" }
        }
    }
}
