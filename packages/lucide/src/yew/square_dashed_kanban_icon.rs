use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SquareDashedKanbanProps {
    #[prop_or(24)]
    pub size: usize,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("none"))]
    pub fill: AttrValue,
    #[prop_or(2)]
    pub stroke_width: usize,
    #[prop_or(false)]
    pub absolute_stroke_width: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: std::option::Option<AttrValue>,
    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[function_component]
pub fn SquareDashedKanban(props: &SquareDashedKanbanProps) -> Html {
    let stroke_width = if props.absolute_stroke_width {
        props.stroke_width * 24 / props.size
    } else {
        props.stroke_width
    };
    
    html! {
        <svg
            ref={props.node_ref.clone()}
            class={classes!("lucide", props.class.clone())}
            style={props.style.clone()}
            xmlns="http://www.w3.org/2000/svg"
            width={props.size.to_string()}
            height={props.size.to_string()}
            viewBox="0 0 24 24"
            fill={&props.fill}
            stroke={&props.color}
            stroke-width={stroke_width.to_string()}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M8 7v7" />
            <path d="M12 7v4" />
            <path d="M16 7v9" />
            <path d="M5 3a2 2 0 0 0-2 2" />
            <path d="M9 3h1" />
            <path d="M14 3h1" />
            <path d="M19 3a2 2 0 0 1 2 2" />
            <path d="M21 9v1" />
            <path d="M21 14v1" />
            <path d="M21 19a2 2 0 0 1-2 2" />
            <path d="M14 21h1" />
            <path d="M9 21h1" />
            <path d="M5 21a2 2 0 0 1-2-2" />
            <path d="M3 14v1" />
            <path d="M3 9v1" />
        </svg>
    }
}
