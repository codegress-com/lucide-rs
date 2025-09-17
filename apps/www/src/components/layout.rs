use crate::components::{Footer, Navbar};
use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white flex flex-col">
            <Navbar />
            <main class="flex-1">
                {children()}
            </main>
            <Footer />
        </div>
    }
}
