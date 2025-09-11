use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_mobile_menu_open, set_mobile_menu_open) = signal(false);

    view! {
        <nav class="bg-white border-b border-neutral-200 sticky top-0 z-50 backdrop-blur-md">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Logo
                    <div class="flex items-center">
                        <A href="/">
                            <div class="flex items-center space-x-3">
                                <div class="w-9 h-9 rounded-xl flex items-center justify-center" style="background: #D34516">
                                    // Lucide zap icon
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2.5">
                                        <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                                    </svg>
                                </div>
                                <span class="font-bold text-xl text-neutral-800">"Lucide Rust"</span>
                            </div>
                        </A>
                    </div>

                    // Desktop Navigation
                    <div class="hidden md:flex items-center space-x-6">
                        <A href="/icons">
                            <div class="flex items-center space-x-2 text-neutral-700 hover:text-[#D34516] px-3 py-2 text-sm font-medium transition-colors">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="3" y="3" width="7" height="7"/>
                                    <rect x="14" y="3" width="7" height="7"/>
                                    <rect x="14" y="14" width="7" height="7"/>
                                    <rect x="3" y="14" width="7" height="7"/>
                                </svg>
                                <span>"Icons"</span>
                            </div>
                        </A>
                        <A href="/get-started">
                            <div class="flex items-center space-x-2 text-neutral-700 hover:text-[#D34516] px-3 py-2 text-sm font-medium transition-colors">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                                    <polyline points="7,10 12,15 17,10"/>
                                    <line x1="12" y1="15" x2="12" y2="3"/>
                                </svg>
                                <span>"Get Started"</span>
                            </div>
                        </A>
                        <A href="/docs">
                            <div class="flex items-center space-x-2 text-neutral-700 hover:text-[#D34516] px-3 py-2 text-sm font-medium transition-colors">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/>
                                </svg>
                                <span>"Docs"</span>
                            </div>
                        </A>
                        <a 
                            href="https://github.com/codegress-com/lucide-rs" 
                            target="_blank"
                            class="inline-flex items-center space-x-2 px-4 py-2 text-sm font-medium text-white bg-neutral-900 hover:bg-neutral-800 rounded-lg transition-colors"
                        >
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/>
                                <path d="M9 18c-4.51 2-5-2-7-2"/>
                            </svg>
                            <span>"GitHub"</span>
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
                        <A href="/get-started">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Get Started"</span>
                        </A>
                        <A href="/docs">
                            <span class="text-gray-700 hover:text-blue-600 px-3 py-2 text-sm font-medium transition-colors">"Documentation"</span>
                        </A>
                        <a 
                            href="https://github.com/codegress-com/lucide-rs" 
                            target="_blank"
                            class="inline-flex items-center justify-center px-4 py-2 bg-neutral-900 text-white text-sm font-medium rounded-lg hover:bg-neutral-800 transition-colors mx-3 mt-2"
                        >
                            "GitHub"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
