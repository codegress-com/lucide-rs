mod pages;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use pages::*;

fn main() {
    mount_to_body(|| {
        view! {
            <div class="min-h-screen bg-gray-100">
                <Router>
                    // Navigation Header
                    <nav class="bg-white shadow-lg border-b">
                        <div class="max-w-7xl mx-auto px-4">
                            <div class="flex justify-between h-16">
                                <div class="flex items-center space-x-8">
                                    <div class="flex items-center space-x-3">
                                        <lucide::Sparkles class="w-8 h-8 text-blue-600" />
                                        <span class="text-xl font-bold text-gray-800">"Lucide Leptos"</span>
                                    </div>
                                    
                                    <div class="hidden md:flex space-x-4">
                                        <a href="/" class="flex items-center space-x-2 px-3 py-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
                                            <lucide::Home class="w-4 h-4" />
                                            <span>"Home"</span>
                                        </a>
                                        <a href="/icons" class="flex items-center space-x-2 px-3 py-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
                                            <lucide::Palette class="w-4 h-4" />
                                            <span>"Icons"</span>
                                        </a>
                                    </div>
                                </div>
                                
                                <div class="flex items-center space-x-4">
                                    <button class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-md">
                                        <lucide::Github class="w-5 h-5" />
                                    </button>
                                    <button class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-md">
                                        <lucide::ExternalLink class="w-5 h-5" />
                                    </button>
                                </div>
                            </div>
                        </div>
                    </nav>

                    <main class="py-8 px-4">
                        <Routes fallback=|| view! { 
                            <div class="text-center py-16">
                                <lucide::AlertCircle class="w-16 h-16 text-red-500 mx-auto mb-4" />
                                <h1 class="text-2xl font-bold text-gray-800 mb-2">"404 - Page Not Found"</h1>
                                <p class="text-gray-600 mb-4">"The page you're looking for doesn't exist."</p>
                                <a href="/" class="inline-flex items-center space-x-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                                    <lucide::ArrowLeft class="w-4 h-4" />
                                    <span>"Go Home"</span>
                                </a>
                            </div>
                        }>
                            <Route path=StaticSegment("") view=HomePage/>
                            <Route path=StaticSegment("icons") view=IconsPage/>
                        </Routes>
                    </main>
                </Router>
            </div>
        }
    })
}
