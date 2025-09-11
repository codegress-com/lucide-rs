use sycamore::prelude::*;

#[derive(Props)]
pub struct Columns3CogProps {
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
pub fn Columns3Cog(props: Columns3CogProps) -> View {
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
            path(d="M10.5 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v5.5")
            path(d="m14.3 19.6 1-.4")
            path(d="M15 3v7.5")
            path(d="m15.2 16.9-.9-.3")
            path(d="m16.6 21.7.3-.9")
            path(d="m16.8 15.3-.4-1")
            path(d="m19.1 15.2.3-.9")
            path(d="m19.6 21.7-.4-1")
            path(d="m20.7 16.8 1-.4")
            path(d="m21.7 19.4-.9-.3")
            path(d="M9 3v18")
            circle(cx="18", cy="18", r="3")
        }
    }
}
