use sycamore::prelude::*;

#[derive(Props)]
pub struct BrainCogProps {
    #[prop(default = 24)]
    pub size: usize,
    #[prop(default = String::from("currentColor"))]
    pub color: String,
    #[prop(default = String::from("none"))]
    pub fill: String,
    #[prop(default = 2)]
    pub stroke_width: usize,
    #[prop(default = false)]
    pub absolute_stroke_width: bool,
    #[prop(default)]
    pub class: Option<String>,
}

#[component]
pub fn BrainCog(props: BrainCogProps) -> View {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    let class = props.class.unwrap_or_default();
    
    view! {
        svg(
            xmlns="http://www.w3.org/2000/svg",
            class=format!("lucide {}", class),
            width=props.size.to_string(),
            height=props.size.to_string(),
            viewBox="0 0 24 24",
            fill=props.fill,
            stroke=props.color,
            "stroke-width"=stroke_width.to_string(),
            "stroke-linecap"="round",
            "stroke-linejoin"="round",
        ) {
            path(d="m10.852 14.772-.383.923")
            path(d="m10.852 9.228-.383-.923")
            path(d="m13.148 14.772.382.924")
            path(d="m13.531 8.305-.383.923")
            path(d="m14.772 10.852.923-.383")
            path(d="m14.772 13.148.923.383")
            path(d="M17.598 6.5A3 3 0 1 0 12 5a3 3 0 0 0-5.63-1.446 3 3 0 0 0-.368 1.571 4 4 0 0 0-2.525 5.771")
            path(d="M17.998 5.125a4 4 0 0 1 2.525 5.771")
            path(d="M19.505 10.294a4 4 0 0 1-1.5 7.706")
            path(d="M4.032 17.483A4 4 0 0 0 11.464 20c.18-.311.892-.311 1.072 0a4 4 0 0 0 7.432-2.516")
            path(d="M4.5 10.291A4 4 0 0 0 6 18")
            path(d="M6.002 5.125a3 3 0 0 0 .4 1.375")
            path(d="m9.228 10.852-.923-.383")
            path(d="m9.228 13.148-.923.383")
            circle(cx="12", cy="12", r="3")
        }
    }
}
