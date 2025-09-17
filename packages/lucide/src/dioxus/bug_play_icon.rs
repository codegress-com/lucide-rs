use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BugPlayProps {
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
pub fn BugPlay(props: BugPlayProps) -> Element {
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
            path { "d": "M10 19.655A6 6 0 0 1 6 14v-3a4 4 0 0 1 4-4h4a4 4 0 0 1 4 3.97" }
            path { "d": "M14 15.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997a1 1 0 0 1-1.517-.86z" }
            path { "d": "M14.12 3.88 16 2" }
            path { "d": "M21 5a4 4 0 0 1-3.55 3.97" }
            path { "d": "M3 21a4 4 0 0 1 3.81-4" }
            path { "d": "M3 5a4 4 0 0 0 3.55 3.97" }
            path { "d": "M6 13H2" }
            path { "d": "m8 2 1.88 1.88" }
            path { "d": "M9 7.13V6a3 3 0 1 1 6 0v1.13" }
        }
    }
}
