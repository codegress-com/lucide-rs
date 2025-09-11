use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_mobile_menu_open, set_mobile_menu_open) = signal(false);

    view! {
        <nav class="bg-white border-b border-gray-200 sticky top-0 z-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Logo
                    <div class="flex items-center">
                        <A href="/">
                            <div class="flex items-center space-x-2">
                                <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                                    <span class="text-white font-bold text-sm">"L"</span>
                                </div>
                                <span class="font-bold text-xl text-gray-900">"Lucide Rust"</span>
                            </div>
                        </A>
                    </div>

                    // Desktop Navigation
                    <div class="hidden md:flex items-center space-x-8">
                        <A href="/icons">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Icons"</span>
                        </A>
                        <A href="/docs">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Documentation"</span>
                        </A>
                        <A href="/docs/installation">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Get Started"</span>
                        </A>
                        <a 
                            href="https://github.com/codegress-com/lucide-rs" 
                            target="_blank"
                            class="inline-flex items-center px-4 py-2 bg-gray-900 text-white text-sm font-medium rounded-lg hover:bg-gray-800 transition-colors"
                        >
                            "GitHub"
                        </a>
                    </div>

                    // Mobile menu button
                    <div class="md:hidden">
                        <button 
                            on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                            class="text-gray-700 hover:text-gray-900 focus:outline-none focus:text-gray-900 p-2"
                        >
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>
                    </div>
                </div>

                // Mobile Navigation Menu
                <div 
                    class=move || {
                        if is_mobile_menu_open.get() {
                            "md:hidden py-4 border-t border-gray-200"
                        } else {
                            "hidden"
                        }
                    }
                >
                    <div class="flex flex-col space-y-2">
                        <A href="/icons">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Icons"</span>
                        </A>
                        <A href="/docs">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Documentation"</span>
                        </A>
                        <A href="/docs/installation">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Get Started"</span>
                        </A>
                        <a 
                            href="https://github.com/codegress-com/lucide-rs" 
                            target="_blank"
                            class="inline-flex items-center justify-center px-4 py-2 bg-gray-900 text-white text-sm font-medium rounded-lg hover:bg-gray-800 transition-colors mx-3 mt-2"
                        >
                            "GitHub"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
