use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DroneProps {
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
pub fn Drone(props: &DroneProps) -> Html {
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
            <path d="M10 10 7 7" />
            <path d="m10 14-3 3" />
            <path d="m14 10 3-3" />
            <path d="m14 14 3 3" />
            <path d="M14.205 4.139a4 4 0 1 1 5.439 5.863" />
            <path d="M19.637 14a4 4 0 1 1-5.432 5.868" />
            <path d="M4.367 10a4 4 0 1 1 5.438-5.862" />
            <path d="M9.795 19.862a4 4 0 1 1-5.429-5.873" />
            <rect x="10" y="8" width="4" height="8" rx="1" />
        </svg>
    }
}
