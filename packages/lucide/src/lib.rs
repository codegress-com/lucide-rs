//! lucide core
//! Enable a framework feature to use icon components.

#[cfg(feature = "dioxus")]
pub mod dioxus;
#[cfg(feature = "leptos")]
pub mod leptos;
#[cfg(feature = "yew")]
pub mod yew;
#[cfg(feature = "sycamore")]
pub mod sycamore;

pub mod types {
    #[derive(Clone, Debug)]
    pub struct IconProps {
        pub size: u32,
        pub color: &'static str,
        pub stroke_width: f32,
        pub class: Option<&'static str>,
    }

    impl Default for IconProps {
        fn default() -> Self {
            Self { size: 24, color: "currentColor", stroke_width: 2.0, class: None }
        }
    }
}
