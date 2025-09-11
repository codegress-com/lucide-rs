use sycamore::prelude::*;

#[derive(Props)]
pub struct BugOffProps {
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
pub fn BugOff(props: BugOffProps) -> View {
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
            path(d="M12 20v-8")
            path(d="M14.12 3.88 16 2")
            path(d="M15 7.13V6a3 3 0 0 0-5.14-2.1L8 2")
            path(d="M18 12.34V11a4 4 0 0 0-4-4h-1.3")
            path(d="m2 2 20 20")
            path(d="M21 5a4 4 0 0 1-3.55 3.97")
            path(d="M22 13h-3.34")
            path(d="M3 21a4 4 0 0 1 3.81-4")
            path(d="M6 13H2")
            path(d="M7.7 7.7A4 4 0 0 0 6 11v3a6 6 0 0 0 11.13 3.13")
        }
    }
}
