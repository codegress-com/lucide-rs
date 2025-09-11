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
            <div class="mb-6 text-sm text-gray-600">
                {move || {
                    let count = filtered_icons().len();
                    format!("{} icons", count)
                }}
            </div>
            
            <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 xl:grid-cols-10 gap-3">
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
                            <div class="text-gray-400 text-4xl mb-4">"🔍"</div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"No icons found"</h3>
                            <p class="text-gray-500">"Try adjusting your search query"</p>
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
    
    let copy_to_clipboard = move |_| {
        let component_name = format!(
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
        <button
            class="group relative flex flex-col items-center p-4 rounded-lg border border-gray-200 hover:border-gray-300 hover:bg-gray-50 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1"
            on:click=copy_to_clipboard
            title=move || format!("Click to copy {} component", icon.display_name)
        >
            <div class="mb-2 text-gray-700 group-hover:text-blue-600 transition-colors">
                <svg 
                    width="24" 
                    height="24" 
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
            
            <div class="text-xs text-center text-gray-600 group-hover:text-gray-900 transition-colors font-medium">
                {icon.display_name}
            </div>
            
            <div class="text-xs text-gray-400 mt-1 opacity-0 group-hover:opacity-100 transition-opacity">
                {icon.name}
            </div>
            
            // Copy feedback
            <div 
                class=move || {
                    if copied.get() {
                        "absolute -top-2 -right-2 bg-green-100 text-green-800 text-xs px-2 py-1 rounded-full transition-all duration-200 opacity-100 scale-100"
                    } else {
                        "absolute -top-2 -right-2 bg-green-100 text-green-800 text-xs px-2 py-1 rounded-full transition-all duration-200 opacity-0 scale-75"
                    }
                }
            >
                "Copied!"
            </div>
        </button>
    }
}
