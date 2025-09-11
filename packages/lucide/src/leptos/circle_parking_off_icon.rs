use leptos::{prelude::*, svg::Svg};

#[component]
pub fn CircleParkingOff(
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
            <path d="M12.656 7H13a3 3 0 0 1 2.984 3.307" />
            <path d="M13 13H9" />
            <path d="M19.071 19.071A1 1 0 0 1 4.93 4.93" />
            <path d="m2 2 20 20" />
            <path d="M8.357 2.687a10 10 0 0 1 12.956 12.956" />
            <path d="M9 17V9" />
        </svg>
    }
}
