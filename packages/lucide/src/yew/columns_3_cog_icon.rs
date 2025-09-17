use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Columns3CogProps {
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
pub fn Columns3Cog(props: &Columns3CogProps) -> Html {
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
            <path d="M10.5 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v5.5" />
            <path d="m14.3 19.6 1-.4" />
            <path d="M15 3v7.5" />
            <path d="m15.2 16.9-.9-.3" />
            <path d="m16.6 21.7.3-.9" />
            <path d="m16.8 15.3-.4-1" />
            <path d="m19.1 15.2.3-.9" />
            <path d="m19.6 21.7-.4-1" />
            <path d="m20.7 16.8 1-.4" />
            <path d="m21.7 19.4-.9-.3" />
            <path d="M9 3v18" />
            <circle cx="18" cy="18" r="3" />
        </svg>
    }
}
