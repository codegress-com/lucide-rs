use crate::components::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <div class="min-h-screen bg-gray-100">
                <Navigation />
                
                <div class="max-w-7xl mx-auto px-4 py-8">
                    // Hero Section
                    <div class="text-center mb-12">
                        <div class="flex justify-center mb-4">
                            <lucide::Sparkles class="w-16 h-16 text-blue-600" />
                        </div>
                        <h1 class="text-4xl font-bold text-gray-900 mb-4">
                            {"Lucide Icons for Yew"}
                        </h1>
                        <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                            {"Beautiful, customizable SVG icons for your Yew applications. "}
                            {"Fast, lightweight, and easy to use."}
                        </p>
                        <div class="mt-8 flex justify-center space-x-4">
                            <button class="flex items-center space-x-2 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                                <lucide::Download class="w-5 h-5" />
                                <span>{"Get Started"}</span>
                            </button>
                            <button class="flex items-center space-x-2 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors">
                                <lucide::Github class="w-5 h-5" />
                                <span>{"View on GitHub"}</span>
                            </button>
                        </div>
                    </div>
                    
                    // Dashboard Stats
                    <DashboardStats />
                    
                    // Features Grid
                    <div class="mb-8">
                        <FeatureGrid />
                    </div>
                    
                    // Icon Showcase
                    <IconShowcase />
                    
                    // Footer
                    <footer class="mt-16 py-8 border-t border-gray-200 text-center">
                        <div class="flex justify-center items-center space-x-2 text-gray-600">
                            <span>{"Built with"}</span>
                            <lucide::Heart class="w-4 h-4 text-red-500" />
                            <span>{"using Yew and Lucide"}</span>
                        </div>
                    </footer>
                </div>
            </div>
        </>
    }
}
