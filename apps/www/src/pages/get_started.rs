use crate::components::Layout;
use leptos::prelude::*;

#[component]
pub fn GetStarted() -> impl IntoView {
    let (active_framework, set_active_framework) = signal(String::from("dioxus"));

    let frameworks = vec![
        ("dioxus", "Dioxus"),
        ("leptos", "Leptos"),
        ("yew", "Yew"),
        ("sycamore", "Sycamore"),
    ];

    let code_examples = move || {
        let framework = active_framework.get();
        match framework.as_ref() {
            "dioxus" => {
                r#"use dioxus::prelude::*;
use lucide_dioxus::Heart;

fn app() -> Element {
    rsx! {
        div {
            Heart { 
                color: "red",
                size: 32 
            }
            p { "Hello from Dioxus!" }
        }
    }
}"#
            }
            "leptos" => {
                r#"use leptos::prelude::*;
use lucide_leptos::Heart;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <Heart 
                color="red"
                size=32
            />
            <p>"Hello from Leptos!"</p>
        </div>
    }
}"#
            }
            "yew" => {
                r#"use yew::prelude::*;
use lucide_yew::Heart;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Heart 
                color="red"
                size=32
            />
            <p>{"Hello from Yew!"}</p>
        </div>
    }
}"#
            }
            "sycamore" => {
                r#"use sycamore::prelude::*;
use lucide_sycamore::Heart;

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        div {
            Heart(
                color = "red",
                size = 32,
            )
            p { "Hello from Sycamore!" }
        }
    }
}"#
            }
            _ => "",
        }
    };

    let icon_categories = vec![
        "Accessibility",
        "Actions",
        "Arrows",
        "Audio",
        "Brand",
        "Communication",
        "Currency",
        "Database",
        "Development",
        "Devices",
        "Editor",
        "Files",
        "Gaming",
        "Gestures",
        "Health",
        "Layout",
        "Location",
        "Lucide",
        "Maps",
        "Media",
        "Navigation",
        "Nature",
        "Notifications",
        "Photography",
        "Security",
        "Shapes",
        "Shopping",
        "Social",
        "Sports",
        "System",
        "Text",
        "Time",
        "Transportation",
        "Travel",
        "Weather",
        "Misc",
    ];

    view! {
        <Layout>
        <div class="min-h-screen bg-white">
            <div class="max-w-7xl mx-auto px-4 py-12">
                // Header
                <div class="text-center mb-12">
                    <h1 class="text-4xl font-bold text-neutral-800 mb-4">
                        "Get Started with Lucide Rust"
                    </h1>
                    <p class="text-xl text-neutral-600 max-w-3xl mx-auto">
                        "Beautiful & consistent icon toolkit for Rust web frameworks. Choose your framework and start using icons in minutes."
                    </p>
                </div>

                // Installation Section
                <div class="mb-16">
                    <h2 class="text-2xl font-bold text-neutral-800 mb-8 text-center">
                        "Installation"
                    </h2>

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

                    // Installation commands
                    <div class="max-w-4xl mx-auto mb-8">
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                            // Basic installation
                            <div>
                                <h3 class="text-sm font-medium text-neutral-700 mb-3">"Basic Installation"</h3>
                                <div class="bg-neutral-900 rounded-lg p-4">
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="text-neutral-400 text-xs font-medium">"Cargo.toml"</span>
                                        <button class="text-neutral-400 hover:text-white text-xs">
                                            "Copy"
                                        </button>
                                    </div>
                                    <pre class="text-green-400 font-mono text-sm">
                                        {move || format!("lucide-{} = \"0.1.0\"", active_framework.get())}
                                    </pre>
                                </div>
                            </div>

                            // With all icons
                            <div>
                                <h3 class="text-sm font-medium text-neutral-700 mb-3">"With All Icons"</h3>
                                <div class="bg-neutral-900 rounded-lg p-4">
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="text-neutral-400 text-xs font-medium">"Cargo.toml"</span>
                                        <button class="text-neutral-400 hover:text-white text-xs">
                                            "Copy"
                                        </button>
                                    </div>
                                    <pre class="text-green-400 font-mono text-sm">
                                        {move || format!("lucide-{} = {{ version = \"0.1.0\", features = [\"all-icons\"] }}", active_framework.get())}
                                    </pre>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Usage example
                    <div class="max-w-2xl mx-auto">
                        <h3 class="text-lg font-semibold text-neutral-800 mb-4 text-center">
                            "Usage Example"
                        </h3>
                        <div class="bg-neutral-900 rounded-lg p-6">
                            <div class="flex items-center justify-between mb-4">
                                <span class="text-neutral-400 text-sm font-medium">
                                    "main.rs"
                                </span>
                                <button class="text-neutral-400 hover:text-white text-sm">
                                    "Copy"
                                </button>
                            </div>
                            <pre class="text-gray-300 font-mono text-sm whitespace-pre-wrap overflow-x-auto">
                                {code_examples}
                            </pre>
                        </div>
                    </div>
                </div>

                // Icon Categories
                <div class="mb-16">
                    <h2 class="text-2xl font-bold text-neutral-800 mb-8 text-center">
                        "Available Icon Categories"
                    </h2>
                    <p class="text-neutral-600 text-center mb-8">
                        "Browse through our extensive collection of icons organized by categories"
                    </p>

                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-3">
                        {icon_categories.into_iter().map(|category| {
                            view! {
                                <div class="bg-gray-50 rounded-lg px-3 py-2 text-center hover:bg-gray-100 transition-colors">
                                    <span class="text-sm font-medium text-neutral-700">
                                        {category}
                                    </span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Quick Links
                <div class="text-center">
                    <h2 class="text-2xl font-bold text-neutral-800 mb-6">
                        "What's Next?"
                    </h2>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a
                            href="/icons"
                            class="inline-flex items-center justify-center px-6 py-3 text-white font-medium rounded-lg transition-colors"
                            style="background: #D34516"
                        >
                            "Browse All Icons"
                        </a>
                        <a
                            href="/docs"
                            class="inline-flex items-center justify-center px-6 py-3 border border-neutral-300 text-neutral-700 font-medium rounded-lg hover:bg-neutral-50 transition-colors"
                        >
                            "Read Documentation"
                        </a>
                    </div>
                </div>
            </div>
        </div>
        </Layout>
    }
}
