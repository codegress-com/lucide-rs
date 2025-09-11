use sycamore::prelude::*;

#[derive(Props)]
pub struct VaultProps {
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
pub fn Vault(props: VaultProps) -> View {
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
            path(d="m7.9 7.9 2.7 2.7")
            path(d="m13.4 10.6 2.7-2.7")
            path(d="m7.9 16.1 2.7-2.7")
            path(d="m13.4 13.4 2.7 2.7")
            rect(x="3", y="3", width="18", height="18")
            circle(cx="7.5", cy="7.5", r=".5")
            circle(cx="16.5", cy="7.5", r=".5")
            circle(cx="7.5", cy="16.5", r=".5")
            circle(cx="16.5", cy="16.5", r=".5")
            circle(cx="12", cy="12", r="2")
        }
    }
}
