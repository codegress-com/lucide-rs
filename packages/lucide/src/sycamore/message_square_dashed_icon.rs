use sycamore::prelude::*;

#[derive(Props)]
pub struct MessageSquareDashedProps {
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
pub fn MessageSquareDashed(props: MessageSquareDashedProps) -> View {
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
            path(d="M12 19h.01")
            path(d="M12 3h.01")
            path(d="M16 19h.01")
            path(d="M16 3h.01")
            path(d="M2 13h.01")
            path(d="M2 17v4.286a.71.71 0 0 0 1.212.502l2.202-2.202A2 2 0 0 1 6.828 19H8")
            path(d="M2 5a2 2 0 0 1 2-2")
            path(d="M2 9h.01")
            path(d="M20 3a2 2 0 0 1 2 2")
            path(d="M22 13h.01")
            path(d="M22 17a2 2 0 0 1-2 2")
            path(d="M22 9h.01")
            path(d="M8 3h.01")
        }
    }
}
