use leptos::{prelude::*, svg::Svg};

#[component]
pub fn Replace(
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
            <path d="M14 4a2 2 0 0 1 2-2" />
            <path d="M16 10a2 2 0 0 1-2-2" />
            <path d="M20 2a2 2 0 0 1 2 2" />
            <path d="M22 8a2 2 0 0 1-2 2" />
            <path d="m3 7 3 3 3-3" />
            <path d="M6 10V5a3 3 0 0 1 3-3h1" />
            <rect x="2" y="14" width="8" height="8" rx="2" />
        </svg>
    }
}
