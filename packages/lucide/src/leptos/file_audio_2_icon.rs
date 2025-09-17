use leptos::{prelude::*, svg::Svg};

#[component]
pub fn FileAudio2(
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
            <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2" />
            <path d="M14 2v4a2 2 0 0 0 2 2h4" />
            <path d="M2 17v-3a4 4 0 0 1 8 0v3" />
            <circle cx="3" cy="17" r="1" />
            <circle cx="9" cy="17" r="1" />
        </svg>
    }
}
