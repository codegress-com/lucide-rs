use leptos::{prelude::*, svg::Svg};

#[component]
pub fn MonitorCog(
    #[prop(default = 24.into(), into)] size: Signal<usize>,
    #[prop(default = "currentColor".into(), into)] color: Signal<String>,
    #[prop(default = "none".into(), into)] fill: Signal<String>,
    #[prop(default = 2.into(), into)] stroke_width: Signal<usize>,
    #[prop(default = false.into(), into)] absolute_stroke_width: Signal<bool>,
    #[prop(optional)] node_ref: NodeRef<Svg>,
) -> impl IntoView {
    let stroke_width = Signal::derive(move || {
        if absolute_stroke_width.get() {
            stroke_width.get() * 24 / size.get()
        } else {
            stroke_width.get()
        }
    });

    view! {
        <svg
            node_ref=node_ref
            class:lucide=true
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || size.get()
            viewBox="0 0 24 24"
            fill=move || fill.get()
            stroke=move || color.get()
            stroke-width=move || stroke_width.get()
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
