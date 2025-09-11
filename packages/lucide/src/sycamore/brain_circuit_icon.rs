use sycamore::prelude::*;

#[derive(Props)]
pub struct BrainCircuitProps {
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
pub fn BrainCircuit(props: BrainCircuitProps) -> View {
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
            path(d="M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z")
            path(d="M9 13a4.5 4.5 0 0 0 3-4")
            path(d="M6.003 5.125A3 3 0 0 0 6.401 6.5")
            path(d="M3.477 10.896a4 4 0 0 1 .585-.396")
            path(d="M6 18a4 4 0 0 1-1.967-.516")
            path(d="M12 13h4")
            path(d="M12 18h6a2 2 0 0 1 2 2v1")
            path(d="M12 8h8")
            path(d="M16 8V5a2 2 0 0 1 2-2")
            circle(cx="16", cy="13", r=".5")
            circle(cx="18", cy="3", r=".5")
            circle(cx="20", cy="21", r=".5")
            circle(cx="20", cy="8", r=".5")
        }
    }
}
