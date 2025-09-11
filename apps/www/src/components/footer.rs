use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-50 border-t border-gray-200">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
                    // Logo & Description
                    <div class="col-span-1 md:col-span-2">
                        <div class="flex items-center space-x-2 mb-4">
                            <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                                <span class="text-white font-bold text-sm">"L"</span>
                            </div>
                            <span class="font-bold text-xl text-gray-900">"Lucide Rust"</span>
                        </div>
                        <p class="text-gray-600 mb-4 max-w-md">
                            "Beautiful Lucide icons as Rust components for Dioxus, Leptos, Yew, and Sycamore web frameworks. Over 1000 icons, tree-shakeable, and developer friendly."
                        </p>
                        <div class="flex space-x-4">
                            <a 
                                href="https://github.com/codegress-com/lucide-rs" 
                                target="_blank"
                                class="text-gray-500 hover:text-gray-700 transition-colors"
                            >
                                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385c.6.105.825-.255.825-.57c0-.285-.015-1.23-.015-2.235c-3.015.555-3.795-.735-4.035-1.41c-.135-.345-.72-1.41-1.23-1.695c-.42-.225-1.02-.78-.015-.795c.945-.015 1.62.87 1.845 1.23c1.08 1.815 2.805 1.305 3.495.99c.105-.78.42-1.305.765-1.605c-2.67-.3-5.46-1.335-5.46-5.925c0-1.305.465-2.385 1.23-3.225c-.12-.3-.54-1.53.12-3.18c0 0 1.005-.315 3.3 1.23c.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23c.66 1.65.24 2.88.12 3.18c.765.84 1.23 1.905 1.23 3.225c0 4.605-2.805 5.625-5.475 5.925c.435.375.81 1.095.81 2.22c0 1.605-.015 2.895-.015 3.3c0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
                                </svg>
                            </a>
                            <a 
                                href="https://twitter.com/codegress" 
                                target="_blank"
                                class="text-gray-500 hover:text-gray-700 transition-colors"
                            >
                                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
                                </svg>
                            </a>
                        </div>
                    </div>

                    // Documentation Links
                    <div>
                        <h3 class="font-semibold text-gray-900 mb-4">"Documentation"</h3>
                        <ul class="space-y-2">
                            <li>
                                <A href="/docs/installation">
                                    <span class="text-gray-600 hover:text-gray-900 transition-colors">"Installation"</span>
                                </A>
                            </li>
                            <li>
                                <A href="/docs/dioxus">
                                    <span class="text-gray-600 hover:text-gray-900 transition-colors">"Dioxus"</span>
                                </A>
                            </li>
                            <li>
                                <A href="/docs/leptos">
                                    <span class="text-gray-600 hover:text-gray-900 transition-colors">"Leptos"</span>
                                </A>
                            </li>
                            <li>
                                <A href="/docs/yew">
                                    <span class="text-gray-600 hover:text-gray-900 transition-colors">"Yew"</span>
                                </A>
                            </li>
                            <li>
                                <A href="/docs/sycamore">
                                    <span class="text-gray-600 hover:text-gray-900 transition-colors">"Sycamore"</span>
                                </A>
                            </li>
                        </ul>
                    </div>

                    // Resources Links
                    <div>
                        <h3 class="font-semibold text-gray-900 mb-4">"Resources"</h3>
                        <ul class="space-y-2">
                            <li>
                                <A href="/icons">
                                    <span class="text-gray-600 hover:text-gray-900 transition-colors">"Icon Library"</span>
                                </A>
                            </li>
                            <li>
                                <a href="https://lucide.dev" target="_blank" class="text-gray-600 hover:text-gray-900 transition-colors">
                                    "Lucide Icons"
                                </a>
                            </li>
                            <li>
                                <a href="https://github.com/codegress-com/lucide-rs" target="_blank" class="text-gray-600 hover:text-gray-900 transition-colors">
                                    "GitHub"
                                </a>
                            </li>
                            <li>
                                <a href="https://crates.io/crates/lucide" target="_blank" class="text-gray-600 hover:text-gray-900 transition-colors">
                                    "Crates.io"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="border-t border-gray-200 mt-8 pt-8 flex flex-col md:flex-row justify-between items-center">
                    <p class="text-gray-600 text-sm">
                        "© 2024 Codegress. Built with Leptos and Tailwind CSS."
                    </p>
                    <div class="mt-4 md:mt-0 flex space-x-6">
                        <a href="https://github.com/codegress-com/lucide-rs/blob/main/LICENSE" target="_blank" class="text-sm text-gray-600 hover:text-gray-900 transition-colors">
                            "MIT License"
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}
