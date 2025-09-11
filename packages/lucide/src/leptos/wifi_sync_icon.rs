use leptos::{prelude::*, svg::Svg};

#[component]
pub fn WifiSync(
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
            <path d="M11.965 10.105v4L13.5 12.5a5 5 0 0 1 8 1.5" />
            <path d="M11.965 14.105h4" />
            <path d="M17.965 18.105h4L20.43 19.71a5 5 0 0 1-8-1.5" />
            <path d="M2 8.82a15 15 0 0 1 20 0" />
            <path d="M21.965 22.105v-4" />
            <path d="M5 12.86a10 10 0 0 1 3-2.032" />
            <path d="M8.5 16.429h.01" />
        </svg>
    }
}
