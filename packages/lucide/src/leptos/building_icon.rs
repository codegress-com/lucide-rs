use leptos::{prelude::*, svg::Svg};

#[component]
pub fn Building(
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
            width=size
            height=size
            viewBox="0 0 24 24"
            fill=fill
            stroke=color
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M12 10h.01" />
            <path d="M12 14h.01" />
            <path d="M12 6h.01" />
            <path d="M16 10h.01" />
            <path d="M16 14h.01" />
            <path d="M16 6h.01" />
            <path d="M8 10h.01" />
            <path d="M8 14h.01" />
            <path d="M8 6h.01" />
            <path d="M9 22v-3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3" />
            <rect x="4" y="2" width="16" height="20" rx="2" />
        </svg>
    }
}
