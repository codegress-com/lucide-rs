use leptos::{prelude::*, svg::Svg};

#[component]
pub fn CircleDotDashed(
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
            <path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0" />
            <path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7" />
            <path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8" />
            <path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69" />
            <path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0" />
            <path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7" />
            <path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8" />
            <path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69" />
            <circle cx="12" cy="12" r="1" />
        </svg>
    }
}
