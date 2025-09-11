use leptos::prelude::*;

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(default = "rust".to_string(), into)] language: String,
    #[prop(default = false)] show_copy: bool,
) -> impl IntoView {
    let (copied, set_copied) = signal(false);

    let copy_to_clipboard = move |_| {
        // Simple copy indication - actual clipboard functionality can be added later
        set_copied.set(true);
        
        // Reset copied state after 2 seconds  
        let handle = set_timeout_with_handle(
            move || set_copied.set(false),
            std::time::Duration::from_secs(2),
        ).expect("Failed to set timeout");
        handle.forget();
    };

    view! {
        <div class="relative bg-gray-900 rounded-lg overflow-hidden">
            {move || if show_copy {
                view! {
                    <div class="absolute top-3 right-3 z-10">
                        <button
                            on:click=copy_to_clipboard
                            class="p-2 text-gray-400 hover:text-white rounded-md hover:bg-gray-800 transition-colors"
                            title="Copy to clipboard"
                        >
                            {move || if copied.get() {
                                view! {
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                }.into_any()
                            } else {
                                view! {
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                    </svg>
                                }.into_any()
                            }}
                        </button>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            <div class="flex items-center justify-between px-4 py-2 bg-gray-800 border-b border-gray-700">
                <span class="text-sm text-gray-300 font-medium">{language.clone()}</span>
                {move || if copied.get() {
                    view! {
                        <span class="text-xs text-green-400">"Copied!"</span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>
            
            <pre class="overflow-x-auto p-4">
                <code class="text-sm text-gray-100 whitespace-pre">
                    {code.clone()}
                </code>
            </pre>
        </div>
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, timeout: i32) -> i32;
    fn clearTimeout(id: i32);
}

pub struct TimeoutHandle(i32);

impl TimeoutHandle {
    pub fn forget(self) {
        std::mem::forget(self);
    }
}

impl Drop for TimeoutHandle {
    fn drop(&mut self) {
        clearTimeout(self.0);
    }
}

pub fn set_timeout_with_handle<F>(f: F, duration: std::time::Duration) -> Result<TimeoutHandle, JsValue>
where
    F: FnOnce() + 'static,
{
    let closure = Closure::once(f);
    let id = setTimeout(&closure, duration.as_millis() as i32);
    closure.forget();
    Ok(TimeoutHandle(id))
}
