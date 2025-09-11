use leptos::prelude::*;
use crate::components::{Layout, SearchBar, IconGrid};

#[component]
pub fn IconsPage() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    
    view! {
        <Layout>
            <div class="bg-white py-16">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-12">
                        <h1 class="text-5xl font-bold text-neutral-800 mb-6">"Icon Library"</h1>
                        <p class="text-xl text-neutral-600 max-w-2xl mx-auto">"Browse and search through our collection of beautiful Lucide icons"</p>
                    </div>
                    
                    <div class="mb-10">
                        <SearchBar search_query=set_search_query />
                    </div>

                    <div class="bg-white rounded-2xl border border-neutral-200 p-8">
                        <IconGrid search_query=search_query.into() />
                    </div>
                </div>
            </div>
        </Layout>
    }
}
