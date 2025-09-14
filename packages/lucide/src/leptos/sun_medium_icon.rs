use leptos::{prelude::*, svg::Svg};

#[component]
pub fn SunMedium(
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
            <path d="M12 3v1" />
            <path d="M12 20v1" />
            <path d="M3 12h1" />
            <path d="M20 12h1" />
            <path d="m18.364 5.636-.707.707" />
            <path d="m6.343 17.657-.707.707" />
            <path d="m5.636 5.636.707.707" />
            <path d="m17.657 17.657.707.707" />
            <circle cx="12" cy="12" r="4" />
        </svg>
    }
}
