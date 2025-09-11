use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::{Layout, CodeBlock};

#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="bg-gray-50 py-16">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-12">
                        <h1 class="text-4xl font-bold text-gray-900 mb-4">"Documentation"</h1>
                        <p class="text-xl text-gray-600">"Everything you need to get started with Lucide Rust"</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <DocCard 
                            title="Installation"
                            description="Get started with Lucide Rust in your project"
                            href="/docs/installation"
                            icon="🚀"
                        />
                        <DocCard 
                            title="Dioxus"
                            description="Using Lucide icons in Dioxus applications"
                            href="/docs/dioxus"
                            icon="⚡"
                        />
                        <DocCard 
                            title="Leptos"
                            description="Integrating icons in Leptos projects"
                            href="/docs/leptos"
                            icon="🦎"
                        />
                        <DocCard 
                            title="Yew"
                            description="Adding icons to your Yew applications"
                            href="/docs/yew"
                            icon="🌿"
                        />
                        <DocCard 
                            title="Sycamore"
                            description="Using icons with the Sycamore framework"
                            href="/docs/sycamore"
                            icon="🌸"
                        />
                        <DocCard 
                            title="Icon Library"
                            description="Browse all available icons"
                            href="/icons"
                            icon="🎨"
                        />
                    </div>
                </div>
            </div>
        </Layout>
    }
}

#[component]
fn DocCard(
    title: &'static str,
    description: &'static str,
    href: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <A href=href>
            <div class="block bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow p-6 h-full">
                <div class="text-3xl mb-4">{icon}</div>
                <h3 class="text-xl font-semibold text-gray-900 mb-2">{title}</h3>
                <p class="text-gray-600">{description}</p>
            </div>
        </A>
    }
}

#[component]
pub fn InstallationPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Installation"</h1>
                
                <div class="space-y-8">
                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Add to your project"</h2>
                        <p class="text-gray-600 mb-4">"Add Lucide Rust to your Cargo.toml:"</p>
                        <CodeBlock 
                            language="toml" 
                            show_copy=true 
                            code="[dependencies]
lucide = { version = \"0.1\", features = [\"leptos\", \"essentials\"] }" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Feature flags"</h2>
                        <p class="text-gray-600 mb-4">"Enable the features you need:"</p>
                        <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-4">
                            <h3 class="font-semibold text-blue-900 mb-2">"Framework features"</h3>
                            <ul class="text-blue-800 text-sm space-y-1">
                                <li>"• leptos - Enable Leptos components"</li>
                                <li>"• dioxus - Enable Dioxus components"</li>
                                <li>"• yew - Enable Yew components"</li>
                                <li>"• sycamore - Enable Sycamore components"</li>
                            </ul>
                        </div>
                        <div class="bg-green-50 border border-green-200 rounded-lg p-4">
                            <h3 class="font-semibold text-green-900 mb-2">"Icon category features"</h3>
                            <ul class="text-green-800 text-sm space-y-1">
                                <li>"• all-icons - Include all icons (large bundle)"</li>
                                <li>"• essentials - Common icons (arrows, navigation, etc.)"</li>
                                <li>"• web-app - Icons for web applications"</li>
                                <li>"• mobile-app - Icons for mobile apps"</li>
                            </ul>
                        </div>
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Next steps"</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <A href="/docs/leptos">
                                <div class="block p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors">
                                    <h3 class="font-semibold text-gray-900 mb-1">"Leptos Guide"</h3>
                                    <p class="text-sm text-gray-600">"Learn how to use icons in Leptos"</p>
                                </div>
                            </A>
                            <A href="/icons">
                                <div class="block p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors">
                                    <h3 class="font-semibold text-gray-900 mb-1">"Browse Icons"</h3>
                                    <p class="text-sm text-gray-600">"Explore all available icons"</p>
                                </div>
                            </A>
                        </div>
                    </section>
                </div>
            </div>
        </Layout>
    }
}

// Framework-specific documentation pages
#[component]
pub fn LeptosDocsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Leptos Integration"</h1>
                
                <div class="space-y-8">
                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Basic Usage"</h2>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use lucide::leptos::zap_icon::Zap;
use leptos::prelude::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Zap size=24 color=\"currentColor\" />
    }
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"With CSS Classes"</h2>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use lucide::leptos::heart_icon::Heart;

view! {
    <Heart 
        size=32 
        color=\"#ef4444\" 
        class=Some(\"hover:text-red-600 transition-colors\") 
    />
}" 
                        />
                    </section>
                </div>
            </div>
        </Layout>
    }
}

#[component] 
pub fn DioxusDocsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Dioxus Integration"</h1>
                
                <div class="space-y-8">
                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Installation"</h2>
                        <p class="text-gray-600 mb-4">"Add Lucide Rust to your Dioxus project:"</p>
                        <CodeBlock 
                            language="toml" 
                            show_copy=true 
                            code="[dependencies]
lucide = { version = \"0.1\", features = [\"dioxus\", \"essentials\"] }
dioxus = \"0.6\"" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Basic Usage"</h2>
                        <p class="text-gray-600 mb-4">"Import and use icons in your Dioxus components:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use dioxus::prelude::*;
use lucide::dioxus::zap_icon::Zap;

#[component]
fn App() -> Element {
    rsx! {
        div {
            h1 { \"My App\" }
            Zap { size: 24, color: \"currentColor\" }
        }
    }
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Icon Properties"</h2>
                        <p class="text-gray-600 mb-4">"All Lucide icons support these properties:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use lucide::dioxus::heart_icon::Heart;

rsx! {
    Heart {
        size: 32,                    // Icon size in pixels
        color: \"#ef4444\",             // Icon color (CSS color)
        stroke_width: 2,             // Stroke width (1-3)
        class: \"hover:text-red-600\", // CSS classes
        style: \"margin: 10px;\",      // Inline styles
        onclick: |_| println!(\"Clicked!\"), // Event handlers
    }
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Complete Example"</h2>
                        <p class="text-gray-600 mb-4">"A complete Dioxus app with Lucide icons:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use dioxus::prelude::*;
use lucide::dioxus::{
    home_icon::Home,
    search_icon::Search,
    user_icon::User,
    settings_icon::Settings
};

#[component]
fn App() -> Element {
    let mut selected = use_signal(|| 0);

    rsx! {
        div { class: \"flex flex-col h-screen\",
            header { class: \"bg-blue-600 text-white p-4 flex items-center gap-2\",
                Home { size: 24, color: \"white\" }
                h1 { \"My Dioxus App\" }
            }
            
            nav { class: \"bg-gray-100 p-4 flex gap-4\",
                button {
                    class: if selected() == 0 { \"text-blue-600\" } else { \"text-gray-600\" },
                    onclick: move |_| selected.set(0),
                    Search { size: 20 }
                    span { \"Search\" }
                }
                button {
                    class: if selected() == 1 { \"text-blue-600\" } else { \"text-gray-600\" },
                    onclick: move |_| selected.set(1),
                    User { size: 20 }
                    span { \"Profile\" }
                }
                button {
                    class: if selected() == 2 { \"text-blue-600\" } else { \"text-gray-600\" },
                    onclick: move |_| selected.set(2),
                    Settings { size: 20 }
                    span { \"Settings\" }
                }
            }
            
            main { class: \"flex-1 p-8\",
                match selected() {
                    0 => rsx! { \"Search content\" },
                    1 => rsx! { \"Profile content\" },
                    2 => rsx! { \"Settings content\" },
                    _ => rsx! { \"Default content\" }
                }
            }
        }
    }
}" 
                        />
                    </section>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn YewDocsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Yew Integration"</h1>
                
                <div class="space-y-8">
                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Installation"</h2>
                        <p class="text-gray-600 mb-4">"Add Lucide Rust to your Yew project:"</p>
                        <CodeBlock 
                            language="toml" 
                            show_copy=true 
                            code="[dependencies]
lucide = { version = \"0.1\", features = [\"yew\", \"essentials\"] }
yew = \"0.21\"" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Basic Usage"</h2>
                        <p class="text-gray-600 mb-4">"Import and use icons in your Yew components:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use yew::prelude::*;
use lucide::yew::zap_icon::Zap;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{ \"My App\" }</h1>
            <Zap size={24} color={\"currentColor\"} />
        </div>
    }
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Icon Properties"</h2>
                        <p class="text-gray-600 mb-4">"All Lucide icons support these properties:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use lucide::yew::heart_icon::Heart;
use yew::prelude::*;

html! {
    <Heart 
        size={32}                         // Icon size in pixels
        color={\"#ef4444\"}                 // Icon color (CSS color)
        stroke_width={2}                  // Stroke width (1-3)
        class={\"hover:text-red-600\"}     // CSS classes
        style={\"margin: 10px;\"}          // Inline styles
        onclick={Callback::from(|_| log!(\"Clicked!\"))} // Event handlers
    />
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Complete Example"</h2>
                        <p class="text-gray-600 mb-4">"A complete Yew app with navigation using Lucide icons:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use yew::prelude::*;
use lucide::yew::{
    home_icon::Home,
    search_icon::Search,
    user_icon::User,
    settings_icon::Settings
};

#[derive(Clone, PartialEq, Eq)]
enum Page {
    Home,
    Search,
    Profile,
    Settings,
}

#[function_component]
fn App() -> Html {
    let current_page = use_state(|| Page::Home);

    let navigate = {
        let current_page = current_page.clone();
        Callback::from(move |page: Page| {
            current_page.set(page);
        })
    };

    html! {
        <div class={\"flex flex-col h-screen\"}>
            <header class={\"bg-blue-600 text-white p-4 flex items-center gap-2\"}>
                <Home size={24} color={\"white\"} />
                <h1>{ \"My Yew App\" }</h1>
            </header>
            
            <nav class={\"bg-gray-100 p-4 flex gap-4\"}>
                <button 
                    class={if *current_page == Page::Search { \"text-blue-600\" } else { \"text-gray-600\" }}
                    onclick={navigate.reform(|_| Page::Search)}
                >
                    <Search size={20} />
                    <span>{ \"Search\" }</span>
                </button>
                <button 
                    class={if *current_page == Page::Profile { \"text-blue-600\" } else { \"text-gray-600\" }}
                    onclick={navigate.reform(|_| Page::Profile)}
                >
                    <User size={20} />
                    <span>{ \"Profile\" }</span>
                </button>
                <button 
                    class={if *current_page == Page::Settings { \"text-blue-600\" } else { \"text-gray-600\" }}
                    onclick={navigate.reform(|_| Page::Settings)}
                >
                    <Settings size={20} />
                    <span>{ \"Settings\" }</span>
                </button>
            </nav>
            
            <main class={\"flex-1 p-8\"}>
                {
                    match &*current_page {
                        Page::Home => html! { <div>{ \"Home content\" }</div> },
                        Page::Search => html! { <div>{ \"Search content\" }</div> },
                        Page::Profile => html! { <div>{ \"Profile content\" }</div> },
                        Page::Settings => html! { <div>{ \"Settings content\" }</div> },
                    }
                }
            </main>
        </div>
    }
}" 
                        />
                    </section>
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn SycamoreDocsPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Sycamore Integration"</h1>
                
                <div class="space-y-8">
                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Installation"</h2>
                        <p class="text-gray-600 mb-4">"Add Lucide Rust to your Sycamore project:"</p>
                        <CodeBlock 
                            language="toml" 
                            show_copy=true 
                            code="[dependencies]
lucide = { version = \"0.1\", features = [\"sycamore\", \"essentials\"] }
sycamore = \"0.9\"" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Basic Usage"</h2>
                        <p class="text-gray-600 mb-4">"Import and use icons in your Sycamore components:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use sycamore::prelude::*;
use lucide::sycamore::zap_icon::Zap;

#[component]
fn App() -> impl IntoView {
    view! {
        div {
            h1 { \"My App\" }
            Zap(size=24, color=\"currentColor\")
        }
    }
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Icon Properties"</h2>
                        <p class="text-gray-600 mb-4">"All Lucide icons support these properties:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use lucide::sycamore::heart_icon::Heart;
use sycamore::prelude::*;

view! {
    Heart(
        size=32,                    // Icon size in pixels
        color=\"#ef4444\",             // Icon color (CSS color)
        stroke_width=2,             // Stroke width (1-3)
        class=\"hover:text-red-600\", // CSS classes
        style=\"margin: 10px;\",      // Inline styles
    )
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"With Reactivity"</h2>
                        <p class="text-gray-600 mb-4">"Using icons with Sycamore's reactive system:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use sycamore::prelude::*;
use lucide::sycamore::{
    heart_icon::Heart,
    heart_filled_icon::HeartFilled
};

#[component]
fn LikeButton() -> impl IntoView {
    let liked = create_signal(false);
    
    let toggle_like = move |_| {
        liked.set(!liked.get());
    };
    
    view! {
        button(
            class=\"flex items-center gap-2 px-4 py-2 rounded-lg transition-colors\",
            class:text-red-600=liked,
            class:text-gray-600=move || !liked.get(),
            on:click=toggle_like
        ) {
            (
                if liked.get() {
                    view! { HeartFilled(size=20, color=\"currentColor\") }
                } else {
                    view! { Heart(size=20, color=\"currentColor\") }
                }
            )
            span { 
                (if liked.get() { \"Liked\" } else { \"Like\" })
            }
        }
    }
}" 
                        />
                    </section>

                    <section>
                        <h2 class="text-2xl font-bold text-gray-900 mb-4">"Complete Example"</h2>
                        <p class="text-gray-600 mb-4">"A complete Sycamore app with navigation using Lucide icons:"</p>
                        <CodeBlock 
                            language="rust" 
                            show_copy=true 
                            code="use sycamore::prelude::*;
use lucide::sycamore::{
    home_icon::Home,
    search_icon::Search,
    user_icon::User,
    settings_icon::Settings
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Search,
    Profile,
    Settings,
}

#[component]
fn App() -> impl IntoView {
    let current_page = create_signal(Page::Home);

    view! {
        div(class=\"flex flex-col h-screen\") {
            header(class=\"bg-blue-600 text-white p-4 flex items-center gap-2\") {
                Home(size=24, color=\"white\")
                h1 { \"My Sycamore App\" }
            }
            
            nav(class=\"bg-gray-100 p-4 flex gap-4\") {
                button(
                    class=\"flex items-center gap-1 px-3 py-2 rounded-lg transition-colors\",
                    class:text-blue-600=move || current_page.get() == Page::Search,
                    class:text-gray-600=move || current_page.get() != Page::Search,
                    on:click=move |_| current_page.set(Page::Search)
                ) {
                    Search(size=20)
                    span { \"Search\" }
                }
                button(
                    class=\"flex items-center gap-1 px-3 py-2 rounded-lg transition-colors\",
                    class:text-blue-600=move || current_page.get() == Page::Profile,
                    class:text-gray-600=move || current_page.get() != Page::Profile,
                    on:click=move |_| current_page.set(Page::Profile)
                ) {
                    User(size=20)
                    span { \"Profile\" }
                }
                button(
                    class=\"flex items-center gap-1 px-3 py-2 rounded-lg transition-colors\",
                    class:text-blue-600=move || current_page.get() == Page::Settings,
                    class:text-gray-600=move || current_page.get() != Page::Settings,
                    on:click=move |_| current_page.set(Page::Settings)
                ) {
                    Settings(size=20)
                    span { \"Settings\" }
                }
            }
            
            main(class=\"flex-1 p-8\") {
                (
                    match current_page.get() {
                        Page::Home => view! { div { \"Home content\" } },
                        Page::Search => view! { div { \"Search content\" } },
                        Page::Profile => view! { div { \"Profile content\" } },
                        Page::Settings => view! { div { \"Settings content\" } },
                    }
                )
            }
        }
    }
}" 
                        />
                    </section>
                </div>
            </div>
        </Layout>
    }
}
