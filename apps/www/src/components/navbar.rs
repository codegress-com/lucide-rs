use leptos::prelude::*;
use leptos_router::components::A;
use lucide::leptos::{
    book_open_icon::BookOpen, download_icon::Download, github_icon::Github,
    layout_grid_icon::LayoutGrid, menu_icon::Menu,
};

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
                            <div class="flex items-center space-x-2 px-3 py-2 text-sm font-medium text-neutral-700 hover:text-[#D34516] transition-colors duration-[25ms]">
                                <LayoutGrid size=16 />
                                <span>"Icons"</span>
                            </div>
                        </A>
                        <A href="/get-started">
                            <div class="flex items-center space-x-2 px-3 py-2 text-sm font-medium text-neutral-700 hover:text-[#D34516] transition-colors duration-[25ms]">
                                <Download size=16 />
                                <span>"Get Started"</span>
                            </div>
                        </A>
                        <A href="/docs">
                            <div class="flex items-center space-x-2 px-3 py-2 text-sm font-medium text-neutral-700 hover:text-[#D34516] transition-colors duration-[25ms]">
                                <BookOpen size=16 />
                                <span>"Docs"</span>
                            </div>
                        </A>
                        <a
                            href="https://github.com/codegress-com/lucide-rs"
                            target="_blank"
                            class="inline-flex items-center space-x-2 px-4 py-2 text-sm font-medium text-white bg-neutral-900 hover:bg-neutral-800 rounded-lg transition-colors duration-150"
                        >
                            <Github size=16 />
                            <span>"GitHub"</span>
                        </a>
                    </div>

                    // Mobile menu button
                    <div class="md:hidden">
                        <button
                            on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                            class="text-gray-700 hover:text-gray-900 focus:outline-none focus:text-gray-900 p-2 transition-colors duration-150"
                        >
                            <Menu size=24 />
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
                            <span class="text-gray-700 hover:text-[#D34516] px-3 py-2 text-sm font-medium transition-colors duration-150">"Icons"</span>
                        </A>
                        <A href="/get-started">
                            <span class="text-gray-700 hover:text-[#D34516] px-3 py-2 text-sm font-medium transition-colors duration-150">"Get Started"</span>
                        </A>
                        <A href="/docs">
                            <span class="text-gray-700 hover:text-[#D34516] px-3 py-2 text-sm font-medium transition-colors duration-150">"Documentation"</span>
                        </A>
                        <a
                            href="https://github.com/codegress-com/lucide-rs"
                            target="_blank"
                            class="inline-flex items-center justify-center px-4 py-2 bg-neutral-900 text-white text-sm font-medium rounded-lg hover:bg-neutral-800 transition-colors duration-150 mx-3 mt-2"
                        >
                            "GitHub"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
