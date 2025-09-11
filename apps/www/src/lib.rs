use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

pub mod components;
pub mod pages;
pub mod data;

use pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Title text="Lucide Rust - Beautiful icons for Rust web frameworks"/>
        <Meta name="description" content="Beautiful Lucide icons as Rust components for Dioxus, Leptos, Yew, and Sycamore web frameworks. 1000+ icons, tree-shakeable, and developer friendly."/>
        <Meta name="keywords" content="rust, lucide, icons, ui, web-components, dioxus, leptos, yew, sycamore, wasm"/>
        <Meta name="author" content="Codegress"/>
        
        // Open Graph
        <Meta property="og:title" content="Lucide Rust - Beautiful icons for Rust web frameworks"/>
        <Meta property="og:description" content="Beautiful Lucide icons as Rust components for Dioxus, Leptos, Yew, and Sycamore web frameworks."/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:site_name" content="Lucide Rust"/>
        
        // Twitter Card
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content="Lucide Rust - Beautiful icons for Rust web frameworks"/>
        <Meta name="twitter:description" content="Beautiful Lucide icons as Rust components for Dioxus, Leptos, Yew, and Sycamore web frameworks."/>

        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=HomePage/>
                <Route path=path!("/icons") view=IconsPage/>
                <Route path=path!("/get-started") view=GetStarted/>
                <Route path=path!("/docs") view=DocsPage/>
                <Route path=path!("/docs/installation") view=InstallationPage/>
                <Route path=path!("/docs/dioxus") view=DioxusDocsPage/>
                <Route path=path!("/docs/leptos") view=LeptosDocsPage/>
                <Route path=path!("/docs/yew") view=YewDocsPage/>
                <Route path=path!("/docs/sycamore") view=SycamoreDocsPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 flex flex-col justify-center items-center">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-900">"404"</h1>
                <p class="text-xl text-gray-600 mt-4">"Page not found"</p>
                <a 
                    href="/" 
                    class="inline-block mt-6 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                >
                    "Go home"
                </a>
            </div>
        </div>
    }
}
