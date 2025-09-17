use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MicOffProps {
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
pub fn MicOff(props: &MicOffProps) -> Html {
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
            <path d="M12 19v3" />
            <path d="M15 9.34V5a3 3 0 0 0-5.68-1.33" />
            <path d="M16.95 16.95A7 7 0 0 1 5 12v-2" />
            <path d="M18.89 13.23A7 7 0 0 0 19 12v-2" />
            <path d="m2 2 20 20" />
            <path d="M9 9v3a3 3 0 0 0 5.12 2.12" />
        </svg>
    }
}
