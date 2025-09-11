use sycamore::prelude::*;

#[derive(Props)]
pub struct SunMediumProps {
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
pub fn SunMedium(props: SunMediumProps) -> View {
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
            path(d="M12 3v1")
            path(d="M12 20v1")
            path(d="M3 12h1")
            path(d="M20 12h1")
            path(d="m18.364 5.636-.707.707")
            path(d="m6.343 17.657-.707.707")
            path(d="m5.636 5.636.707.707")
            path(d="m17.657 17.657.707.707")
            circle(cx="12", cy="12", r="4")
        }
    }
}
