use leptos::{prelude::*, svg::Svg};

#[component]
pub fn BugOff(
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
            <path d="M12 20v-8" />
            <path d="M14.12 3.88 16 2" />
            <path d="M15 7.13V6a3 3 0 0 0-5.14-2.1L8 2" />
            <path d="M18 12.34V11a4 4 0 0 0-4-4h-1.3" />
            <path d="m2 2 20 20" />
            <path d="M21 5a4 4 0 0 1-3.55 3.97" />
            <path d="M22 13h-3.34" />
            <path d="M3 21a4 4 0 0 1 3.81-4" />
            <path d="M6 13H2" />
            <path d="M7.7 7.7A4 4 0 0 0 6 11v3a6 6 0 0 0 11.13 3.13" />
        </svg>
    }
}
