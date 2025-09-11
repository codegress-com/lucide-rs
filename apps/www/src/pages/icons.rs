use leptos::prelude::*;
use crate::components::{Layout, SearchBar, IconGrid};

#[component]
pub fn IconsPage() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    
    view! {
        <Layout>
            <div class="bg-gray-50 py-16">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-8">
                        <h1 class="text-4xl font-bold text-gray-900 mb-4">"Icon Library"</h1>
                        <p class="text-xl text-gray-600">"Browse and search through our collection of beautiful Lucide icons"</p>
                    </div>
                    
                    <div class="mb-8">
                        <SearchBar search_query=set_search_query />
                    </div>

                    <div class="bg-white rounded-lg shadow-sm p-6">
                        <IconGrid search_query=search_query.into() />
                    </div>
                </div>
            </div>
        </Layout>
    }
}
