use leptos::prelude::*;
use crate::components::{Layout, CodeBlock};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Layout>
            <section class="bg-gradient-to-b from-gray-50 to-white">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                    <div class="text-center">
                        <div class="inline-flex items-center space-x-3 bg-blue-50 text-blue-700 px-4 py-2 rounded-full text-sm font-medium mb-6">
                            <span>"Lucide Rust"</span>
                            <span class="text-blue-500">"1000+ icons"</span>
                            <span>"for Rust UI frameworks"</span>
                        </div>
                        <h1 class="text-5xl font-extrabold text-gray-900 mb-6">
                            "Beautiful Lucide icons for Rust web apps"
                        </h1>
                        <p class="text-xl text-gray-600 max-w-3xl mx-auto mb-8">
                            "Use Lucide icons as native Rust components in Dioxus, Leptos, Yew, and Sycamore. Tree-shakeable, customizable, and fast."
                        </p>
                        <div class="flex justify-center space-x-4">
                            <a href="/docs/installation" class="px-6 py-3 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors">"Get Started"</a>
                            <a href="/icons" class="px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors">"Browse Icons"</a>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-16">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
                        <div>
                            <h2 class="text-2xl font-bold text-gray-900 mb-4">"Installation"</h2>
                            <p class="text-gray-600 mb-4">"Add the crate to your project:"</p>
                            <CodeBlock language="bash" show_copy=true code="cargo add lucide" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold text-gray-900 mb-4">"Usage (Leptos)"</h2>
                            <p class="text-gray-600 mb-4">"Example of using an icon in Leptos:"</p>
                            <CodeBlock language="rust" show_copy=true code="use lucide::leptos::zap_icon::Zap;

#[component]
pub fn Example() -> impl IntoView {
    view! { <Zap size=24 color=\"#111827\" class=Some(\"w-6 h-6\") /> }
}" />
                        </div>
                    </div>
                </div>
            </section>
        </Layout>
    }
}

