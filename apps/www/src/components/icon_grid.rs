use leptos::prelude::*;
use crate::data::{IconInfo, search_icons};

#[component]
pub fn IconGrid(search_query: ReadSignal<String>) -> impl IntoView {
    let filtered_icons = move || {
        let query = search_query.get();
        search_icons(&query)
    };
    
    view! {
        <div>
            <div class="mb-6 text-sm text-neutral-600">
                {move || {
                    let count = filtered_icons().len();
                    format!("{} icons", count)
                }}
            </div>
            
            <div class="grid grid-cols-8 sm:grid-cols-10 md:grid-cols-12 lg:grid-cols-14 xl:grid-cols-16 gap-4">
                <For
                    each=filtered_icons
                    key=|icon| icon.name
                    children=move |icon: &IconInfo| {
                        view! {
                            <IconCard icon=icon.clone() />
                        }
                    }
                />
            </div>
            
            {move || {
                let icons = filtered_icons();
                if icons.is_empty() {
                    view! {
                        <div class="text-center py-16">
                            <div class="text-neutral-400 text-4xl mb-4">"🔍"</div>
                            <h3 class="text-lg font-medium text-neutral-800 mb-2">"No icons found"</h3>
                            <p class="text-neutral-500">"Try adjusting your search query"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn IconCard(icon: IconInfo) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let (is_hovered, set_hovered) = signal(false);
    
    let copy_to_clipboard = move |_| {
        let _component_name = format!(
            "<{} size={{24}} />", 
            icon.display_name.replace(" ", "")
        );
        
        // Simple clipboard copy (for demo purposes)
        set_copied.set(true);
        
        // Reset copied state after 2 seconds
        set_timeout(
            move || set_copied.set(false),
            std::time::Duration::from_millis(2000),
        );
    };
    
    view! {
        <div class="relative">
            <button
                class="group relative flex items-center justify-center w-12 h-12 rounded-lg bg-gray-50 hover:bg-gray-100 border border-transparent hover:border-gray-200 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-blue-500"
                on:click=copy_to_clipboard
                on:mouseenter=move |_| set_hovered.set(true)
                on:mouseleave=move |_| set_hovered.set(false)
            >
                <div class="text-neutral-600 group-hover:text-neutral-800 transition-colors">
                    <svg 
                        width="20" 
                        height="20" 
                        viewBox="0 0 24 24" 
                        fill="none" 
                        stroke="currentColor" 
                        stroke-width="2" 
                        stroke-linecap="round" 
                        stroke-linejoin="round"
                    >
                        <path d=icon.svg_path></path>
                    </svg>
                </div>
            </button>
            
            // Hover tooltip
            <div 
                class=move || {
                    if is_hovered.get() {
                        "absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-gray-900 text-white text-sm rounded-md shadow-lg z-10 whitespace-nowrap pointer-events-none opacity-100 translate-y-0 transition-all duration-200"
                    } else {
                        "absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-gray-900 text-white text-sm rounded-md shadow-lg z-10 whitespace-nowrap pointer-events-none opacity-0 translate-y-1 transition-all duration-200"
                    }
                }
            >
                {icon.display_name}
                // Tooltip arrow
                <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-l-transparent border-r-transparent border-t-gray-900"></div>
            </div>
            
            // Copy feedback
            <div 
                class=move || {
                    if copied.get() {
                        "absolute -top-1 -right-1 text-white text-xs px-2 py-1 rounded-full transition-all duration-200 opacity-100 scale-100 shadow-lg"
                    } else {
                        "absolute -top-1 -right-1 text-white text-xs px-2 py-1 rounded-full transition-all duration-200 opacity-0 scale-75"
                    }
                }
                style=move || {
                    if copied.get() {
                        "background: #D34516"
                    } else {
                        "background: #D34516"
                    }
                }
            >
                "✓"
            </div>
        </div>
    }
}
