use leptos::{prelude::*, svg::Svg};

#[component]
pub fn MailSearch(
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
            <path d="M22 12.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h7.5" />
            <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
            <path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
            <path d="m22 22-1.5-1.5" />
            <circle cx="18" cy="18" r="3" />
        </svg>
    }
}
