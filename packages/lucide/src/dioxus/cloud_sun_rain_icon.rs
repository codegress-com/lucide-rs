use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CloudSunRainProps {
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
pub fn CloudSunRain(props: CloudSunRainProps) -> Element {
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
            path { "d": "M12 2v2" }
            path { "d": "m4.93 4.93 1.41 1.41" }
            path { "d": "M20 12h2" }
            path { "d": "m19.07 4.93-1.41 1.41" }
            path { "d": "M15.947 12.65a4 4 0 0 0-5.925-4.128" }
            path { "d": "M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" }
            path { "d": "M11 20v2" }
            path { "d": "M7 19v2" }
        }
    }
}
