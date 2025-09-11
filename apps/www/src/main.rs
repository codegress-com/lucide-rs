use leptos::prelude::*;
use lucide_docs::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    
    log("WASM module started");

    // Hide loading indicator immediately
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                let _ = body.class_list().add_1("loaded");
                log("Added loaded class to body");
            }
            
            // Also directly hide the loading element
            if let Some(loading_el) = document.get_element_by_id("loading") {
                let _ = loading_el.set_attribute("style", "display: none !important;");
                log("Loading indicator hidden directly");
            }
        }
    }

    // Mount the app
    leptos::mount::mount_to_body(App);
    log("App mounted");
}
