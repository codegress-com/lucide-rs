use crate::components::Layout;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (active_framework, set_active_framework) = signal(String::from("leptos"));

    let frameworks = vec![
        ("dioxus", "Dioxus"),
        ("leptos", "Leptos"),
        ("yew", "Yew"),
        ("sycamore", "Sycamore"),
    ];

    let usage_examples = move || {
        let framework = active_framework.get();
        match framework.as_ref() {
            "dioxus" => {
                "use dioxus::prelude::*;
use lucide_dioxus::Heart;

fn app() -> Element {
    rsx! {
        div {
            Heart { 
                size: 24,
                color: \"currentColor\" 
            }
        }
    }
}"
            }
            "leptos" => {
                "use leptos::prelude::*;
use lucide_leptos::Heart;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Heart size=24 color=\"currentColor\" />
    }
}"
            }
            "yew" => {
                "use yew::prelude::*;
use lucide_yew::Heart;

#[function_component]
fn App() -> Html {
    html! {
        <Heart size=24 color=\"currentColor\" />
    }
}"
            }
            "sycamore" => {
                "use sycamore::prelude::*;
use lucide_sycamore::Heart;

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        Heart(size = 24, color = \"currentColor\")
    }
}"
            }
            _ => "",
        }
    };
    view! {
        <Layout>
            <section class="bg-white relative overflow-hidden">
                // Subtle background pattern
                <div class="absolute inset-0 bg-[radial-gradient(circle_at_30%_40%,rgba(211,69,22,0.05),transparent_40%),radial-gradient(circle_at_70%_60%,rgba(211,69,22,0.03),transparent_40%)]" />

                <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-24 relative">
                    <div class="text-center">
                        <div class="inline-flex items-center space-x-2 px-4 py-2 rounded-full text-sm font-medium mb-8" style="background: rgba(211,69,22,0.1); color: #D34516">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                            </svg>
                            <span>"Lucide Rust"</span>
                            <span class="px-2 py-1 rounded-full text-xs" style="background: #D34516; color: white">"80+ icons"</span>
                        </div>

                        <h1 class="text-5xl md:text-6xl lg:text-7xl font-bold text-neutral-800 mb-8 leading-tight">
                            "Beautiful & consistent"
                            <br/>
                            <span style="color: #D34516">"icons"</span>
                        </h1>

                        <p class="text-xl text-neutral-600 max-w-3xl mx-auto mb-12 leading-relaxed">
                            "Beautiful Lucide icons as native Rust components for modern web frameworks. "
                            "Tree-shakeable, customizable, and developer-friendly."
                        </p>

                        <div class="flex flex-col sm:flex-row justify-center items-center gap-4 mb-16">
                            <a
                                href="/icons"
                                class="inline-flex items-center space-x-2 px-8 py-4 rounded-xl font-semibold text-white transition-all duration-200 transform hover:scale-105 shadow-lg hover:shadow-xl"
                                style="background: linear-gradient(135deg, #D34516 0%, #be3e14 100%)"
                            >
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <rect x="3" y="3" width="7" height="7"/>
                                    <rect x="14" y="3" width="7" height="7"/>
                                    <rect x="14" y="14" width="7" height="7"/>
                                    <rect x="3" y="14" width="7" height="7"/>
                                </svg>
                                <span>"View all icons"</span>
                            </a>

                            <a
                                href="/get-started"
                                class="inline-flex items-center space-x-2 px-8 py-4 bg-neutral-50 hover:bg-neutral-100 border border-neutral-200 text-neutral-700 hover:text-neutral-900 rounded-xl font-semibold transition-all duration-200 hover:shadow-lg hover:border-neutral-300"
                            >
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                                    <polyline points="7,10 12,15 17,10"/>
                                    <line x1="12" y1="15" x2="12" y2="3"/>
                                </svg>
                                <span>"Get Started"</span>
                            </a>
                        </div>

                        // Show some example icons
                        <div class="flex justify-center items-center space-x-8 opacity-60">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
                                <circle cx="12" cy="7" r="4"/>
                            </svg>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
                                <polyline points="9,22 9,12 15,12 15,22"/>
                            </svg>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
                            </svg>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                <polygon points="12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26"/>
                            </svg>
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                <path d="m21 21-6-6m2-5a7 7 0 1 1-14 0 7 7 0 0 1 14 0z"/>
                            </svg>
                        </div>
                    </div>
                </div>
            </section>

            // Features Section
            <section style="background: linear-gradient(135deg, #fafafa 0%, rgba(211,69,22,0.02) 100%)" class="py-24">
                <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-4xl font-bold text-neutral-800 mb-4">"Why Lucide Rust?"</h2>
                        <p class="text-xl text-neutral-600 max-w-2xl mx-auto">"The perfect icon library for Rust web applications"</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-20">
                        <div class="text-center group">
                            <div class="w-20 h-20 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-transform duration-200 group-hover:scale-110" style="background: rgba(211,69,22,0.1)">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                    <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-neutral-800 mb-3">"Lightweight & Scalable"</h3>
                            <p class="text-neutral-600 leading-relaxed">"Icons are lightweight, highly optimized scalable vector graphics (SVG). Perfect for modern web applications."</p>
                        </div>

                        <div class="text-center group">
                            <div class="w-20 h-20 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-transform duration-200 group-hover:scale-110" style="background: rgba(211,69,22,0.1)">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                    <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-neutral-800 mb-3">"Clean & Consistent"</h3>
                            <p class="text-neutral-600 leading-relaxed">"Designed with a strict set of design rules for consistency in style and readability across all icons."</p>
                        </div>

                        <div class="text-center group">
                            <div class="w-20 h-20 rounded-2xl flex items-center justify-center mx-auto mb-6 transition-transform duration-200 group-hover:scale-110" style="background: rgba(211,69,22,0.1)">
                                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#D34516" stroke-width="2">
                                    <path d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"/>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-neutral-800 mb-3">"Fully Customizable"</h3>
                            <p class="text-neutral-600 leading-relaxed">"Customize the color, size, stroke width, and more with simple props. Perfect for any design system."</p>
                        </div>
                    </div>

                    // Framework examples with tabs
                    <div class="max-w-4xl mx-auto">
                        <h2 class="text-3xl font-bold text-neutral-800 mb-6 text-center">"Choose Your Framework"</h2>
                        <p class="text-neutral-600 mb-8 text-center">"Get started with your favorite Rust web framework"</p>

                        // Framework tabs
                        <div class="flex justify-center mb-8">
                            <div class="inline-flex rounded-lg border border-neutral-200 bg-neutral-50 p-1">
                                {frameworks.iter().map(|(key, name)| {
                                    let key_str = *key;
                                    let is_active = move || active_framework.get() == key_str;

                                    view! {
                                        <button
                                            class=move || format!(
                                                "px-4 py-2 text-sm font-medium rounded-md transition-colors {}",
                                                if is_active() {
                                                    "text-white shadow-sm"
                                                } else {
                                                    "text-neutral-600 hover:text-neutral-800"
                                                }
                                            )
                                            style=move || if is_active() {
                                                "background: #D34516"
                                            } else {
                                                ""
                                            }
                                            on:click=move |_| set_active_framework.set(key_str.to_string())
                                        >
                                            {*name}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>

                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                            <div>
                                <h3 class="text-xl font-semibold text-neutral-800 mb-4">"Installation"</h3>
                                <div class="bg-neutral-900 rounded-lg p-4">
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="text-neutral-400 text-xs font-medium">"Cargo.toml"</span>
                                        <button class="text-neutral-400 hover:text-white text-xs">"Copy"</button>
                                    </div>
                                    <pre class="text-green-400 font-mono text-sm">
                                        {move || format!("lucide-{} = \"0.1.0\"", active_framework.get())}
                                    </pre>
                                </div>
                            </div>
                            <div>
                                <h3 class="text-xl font-semibold text-neutral-800 mb-4">"Usage Example"</h3>
                                <div class="bg-neutral-900 rounded-lg p-4">
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="text-neutral-400 text-xs font-medium">"main.rs"</span>
                                        <button class="text-neutral-400 hover:text-white text-xs">"Copy"</button>
                                    </div>
                                    <pre class="text-gray-300 font-mono text-sm whitespace-pre-wrap overflow-x-auto">
                                        {usage_examples}
                                    </pre>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </Layout>
    }
}
