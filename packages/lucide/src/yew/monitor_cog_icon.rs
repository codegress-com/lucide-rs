use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MonitorCogProps {
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
pub fn MonitorCog(props: &MonitorCogProps) -> Html {
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
            <path d="M12 17v4" />
            <path d="m14.305 7.53.923-.382" />
            <path d="m15.228 4.852-.923-.383" />
            <path d="m16.852 3.228-.383-.924" />
            <path d="m16.852 8.772-.383.923" />
            <path d="m19.148 3.228.383-.924" />
            <path d="m19.53 9.696-.382-.924" />
            <path d="m20.772 4.852.924-.383" />
            <path d="m20.772 7.148.924.383" />
            <path d="M22 13v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" />
            <path d="M8 21h8" />
            <circle cx="18" cy="6" r="3" />
        </svg>
    }
}
