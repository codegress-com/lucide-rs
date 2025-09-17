use leptos::prelude::*;

#[component]
pub fn SearchBar(search_query: WriteSignal<String>) -> impl IntoView {
    let (current_value, set_current_value) = signal(String::new());

    // Sync the local state with the search query
    Effect::new(move |_| {
        search_query.set(current_value.get());
    });

    view! {
        <div class="relative max-w-md mx-auto">
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <svg class="h-5 w-5 text-neutral-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
            </div>
            <input
                type="text"
                placeholder="Search icons..."
                class="block w-full pl-10 pr-3 py-4 border-2 border-neutral-200 rounded-xl focus:ring-2 focus:border-transparent transition-all duration-200" style="focus:ring-color: rgba(211,69,22,0.2); focus:border-color: #D34516"
                prop:value=move || current_value.get()
                on:input=move |e| {
                    set_current_value.set(event_target_value(&e));
                }
            />
        </div>
    }
}
