mod components;

use components::*;
use dioxus::html::global_attributes::class;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/showcase")]
    Showcase {},
    #[route("/dashboard")]
    Dashboard {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Add Tailwind CSS
        document::Link { rel: "stylesheet", href: "https://cdn.tailwindcss.com" }
        div { 
            class: "min-h-screen bg-gray-50",
            Router::<Route> {}
        }
    }
}

/// Home page with hero section
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto px-4 py-12",
            // Hero Section
            div { class: "text-center py-20",
                div { class: "flex justify-center mb-8",
                    lucide::Rocket { class: "w-20 h-20 text-blue-600" }
                }
                h1 { class: "text-6xl font-bold text-gray-900 mb-6",
                    "Lucide Icons for Dioxus"
                }
                p { class: "text-xl text-gray-600 max-w-3xl mx-auto mb-12",
                    "Beautiful, customizable SVG icons for modern Dioxus applications. "
                    "Fast, lightweight, and developer-friendly with reactive components."
                }
                
                div { class: "flex justify-center space-x-6",
                    button { class: "flex items-center space-x-3 px-8 py-4 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-colors shadow-lg",
                        lucide::Download { class: "w-6 h-6" }
                        span { class: "font-semibold", "Get Started" }
                    }
                    button { class: "flex items-center space-x-3 px-8 py-4 border-2 border-gray-300 text-gray-700 rounded-xl hover:bg-gray-100 transition-colors",
                        lucide::Github { class: "w-6 h-6" }
                        span { class: "font-semibold", "View on GitHub" }
                    }
                }
            }
            
            // Features Grid
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 mb-20",
                FeatureCard {
                    icon: rsx! { lucide::Zap { class: "w-12 h-12 text-yellow-600" } },
                    title: "Lightning Fast",
                    description: "Optimized for performance with minimal bundle impact"
                }
                FeatureCard {
                    icon: rsx! { lucide::Palette { class: "w-12 h-12 text-purple-600" } },
                    title: "Customizable",
                    description: "Easy to style with classes, props, and CSS variables"
                }
                FeatureCard {
                    icon: rsx! { lucide::TreePine { class: "w-12 h-12 text-green-600" } },
                    title: "Tree Shakeable",
                    description: "Only include the icons you use with category-based features"
                }
            }
            
            // Popular Icons Preview
            div { class: "bg-white rounded-2xl shadow-lg p-8",
                div { class: "text-center mb-8",
                    h2 { class: "text-3xl font-bold text-gray-900 mb-4",
                        "Popular Icons"
                    }
                    p { class: "text-gray-600",
                        "A selection of commonly used icons in modern applications"
                    }
                }
                
                div { class: "grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 gap-6",
                    PopularIcon { icon: rsx! { lucide::Home { class: "w-8 h-8" } }, name: "Home" }
                    PopularIcon { icon: rsx! { lucide::User { class: "w-8 h-8" } }, name: "User" }
                    PopularIcon { icon: rsx! { lucide::Settings { class: "w-8 h-8" } }, name: "Settings" }
                    PopularIcon { icon: rsx! { lucide::Search { class: "w-8 h-8" } }, name: "Search" }
                    PopularIcon { icon: rsx! { lucide::Bell { class: "w-8 h-8" } }, name: "Bell" }
                    PopularIcon { icon: rsx! { lucide::Mail { class: "w-8 h-8" } }, name: "Mail" }
                    PopularIcon { icon: rsx! { lucide::Heart { class: "w-8 h-8" } }, name: "Heart" }
                    PopularIcon { icon: rsx! { lucide::Star { class: "w-8 h-8" } }, name: "Star" }
                    PopularIcon { icon: rsx! { lucide::Download { class: "w-8 h-8" } }, name: "Download" }
                    PopularIcon { icon: rsx! { lucide::Upload { class: "w-8 h-8" } }, name: "Upload" }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: Element, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "text-center p-8 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-shadow",
            div { class: "flex justify-center mb-6", {icon} }
            h3 { class: "text-xl font-semibold text-gray-900 mb-4", "{title}" }
            p { class: "text-gray-600", "{description}" }
        }
    }
}

#[component]
fn PopularIcon(icon: Element, name: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group",
            div { class: "text-gray-700 group-hover:text-blue-600 transition-colors mb-2",
                {icon}
            }
            span { class: "text-xs text-gray-600 text-center font-medium", "{name}" }
        }
    }
}

/// Icon showcase page
#[component]
fn Showcase() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto px-4 py-8",
            div { class: "text-center mb-12",
                h1 { class: "text-4xl font-bold text-gray-900 mb-4",
                    "Icon Showcase"
                }
                p { class: "text-xl text-gray-600",
                    "Explore our comprehensive collection of icons organized by category"
                }
            }
            
            // Navigation Icons
            CategorySection {
                title: "Navigation & UI",
                icons: rsx! {
                    ShowcaseIcon { icon: rsx! { lucide::Home { class: "w-8 h-8" } }, name: "Home" }
                    ShowcaseIcon { icon: rsx! { lucide::Menu { class: "w-8 h-8" } }, name: "Menu" }
                    ShowcaseIcon { icon: rsx! { lucide::ArrowLeft { class: "w-8 h-8" } }, name: "Arrow Left" }
                    ShowcaseIcon { icon: rsx! { lucide::ArrowRight { class: "w-8 h-8" } }, name: "Arrow Right" }
                    ShowcaseIcon { icon: rsx! { lucide::ChevronDown { class: "w-8 h-8" } }, name: "Chevron Down" }
                    ShowcaseIcon { icon: rsx! { lucide::X { class: "w-8 h-8" } }, name: "Close" }
                }
            }
            
            // Communication Icons
            CategorySection {
                title: "Communication",
                icons: rsx! {
                    ShowcaseIcon { icon: rsx! { lucide::Mail { class: "w-8 h-8" } }, name: "Mail" }
                    ShowcaseIcon { icon: rsx! { lucide::MessageCircle { class: "w-8 h-8" } }, name: "Message" }
                    ShowcaseIcon { icon: rsx! { lucide::Phone { class: "w-8 h-8" } }, name: "Phone" }
                    ShowcaseIcon { icon: rsx! { lucide::Video { class: "w-8 h-8" } }, name: "Video" }
                    ShowcaseIcon { icon: rsx! { lucide::Send { class: "w-8 h-8" } }, name: "Send" }
                    ShowcaseIcon { icon: rsx! { lucide::AtSign { class: "w-8 h-8" } }, name: "At Sign" }
                }
            }
            
            // File Icons
            CategorySection {
                title: "Files & Storage",
                icons: rsx! {
                    ShowcaseIcon { icon: rsx! { lucide::File { class: "w-8 h-8" } }, name: "File" }
                    ShowcaseIcon { icon: rsx! { lucide::Folder { class: "w-8 h-8" } }, name: "Folder" }
                    ShowcaseIcon { icon: rsx! { lucide::Download { class: "w-8 h-8" } }, name: "Download" }
                    ShowcaseIcon { icon: rsx! { lucide::Upload { class: "w-8 h-8" } }, name: "Upload" }
                    ShowcaseIcon { icon: rsx! { lucide::Save { class: "w-8 h-8" } }, name: "Save" }
                    ShowcaseIcon { icon: rsx! { lucide::Share { class: "w-8 h-8" } }, name: "Share" }
                }
            }
        }
    }
}

#[component]
fn CategorySection(title: &'static str, icons: Element) -> Element {
    rsx! {
        div { class: "bg-white rounded-2xl shadow-lg p-8 mb-8",
            h2 { class: "text-2xl font-bold text-gray-900 mb-6", "{title}" }
            div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6",
                {icons}
            }
        }
    }
}

#[component]
fn ShowcaseIcon(icon: Element, name: &'static str) -> Element {
    rsx! {
        div { class: "flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group",
            div { class: "text-gray-700 group-hover:text-blue-600 transition-colors mb-3",
                {icon}
            }
            span { class: "text-sm text-gray-600 text-center font-medium", "{name}" }
        }
    }
}

/// Dashboard demo page
#[component]
fn Dashboard() -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto px-4 py-8",
            div { class: "mb-8",
                h1 { class: "text-3xl font-bold text-gray-900 mb-2",
                    "Dashboard Example"
                }
                p { class: "text-gray-600",
                    "A sample dashboard showing practical icon usage in a real application"
                }
            }
            
            // Stats Cards
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8",
                StatCard {
                    icon: rsx! { lucide::Users { class: "w-8 h-8 text-blue-600" } },
                    title: "Users",
                    value: "12,345",
                    change: "+12%",
                    positive: true
                }
                StatCard {
                    icon: rsx! { lucide::DollarSign { class: "w-8 h-8 text-green-600" } },
                    title: "Revenue",
                    value: "$54,321",
                    change: "+8%",
                    positive: true
                }
                StatCard {
                    icon: rsx! { lucide::TrendingUp { class: "w-8 h-8 text-purple-600" } },
                    title: "Growth",
                    value: "23.5%",
                    change: "+2.1%",
                    positive: true
                }
                StatCard {
                    icon: rsx! { lucide::Activity { class: "w-8 h-8 text-orange-600" } },
                    title: "Activity",
                    value: "89%",
                    change: "-1.2%",
                    positive: false
                }
            }
            
            // Quick Actions
            div { class: "bg-white rounded-2xl shadow-lg p-8",
                h2 { class: "text-2xl font-bold text-gray-900 mb-6",
                    "Quick Actions"
                }
                div { class: "grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4",
                    ActionButton { icon: rsx! { lucide::Plus { class: "w-6 h-6" } }, label: "Add User" }
                    ActionButton { icon: rsx! { lucide::FileText { class: "w-6 h-6" } }, label: "New Report" }
                    ActionButton { icon: rsx! { lucide::Settings { class: "w-6 h-6" } }, label: "Settings" }
                    ActionButton { icon: rsx! { lucide::Download { class: "w-6 h-6" } }, label: "Export" }
                    ActionButton { icon: rsx! { lucide::Upload { class: "w-6 h-6" } }, label: "Import" }
                    ActionButton { icon: rsx! { lucide::Refresh { class: "w-6 h-6" } }, label: "Refresh" }
                }
            }
        }
    }
}

#[component]
fn StatCard(icon: Element, title: &'static str, value: &'static str, change: &'static str, positive: bool) -> Element {
    let change_color = if positive { "text-green-600" } else { "text-red-600" };
    let change_icon = if positive {
        rsx! { lucide::ArrowUp { class: "w-4 h-4" } }
    } else {
        rsx! { lucide::ArrowDown { class: "w-4 h-4" } }
    };
    
    rsx! {
        div { class: "bg-white p-6 rounded-xl shadow-lg border border-gray-200",
            div { class: "flex items-center justify-between mb-4",
                div { class: "p-2 bg-gray-50 rounded-lg", {icon} }
                div { class: "flex items-center space-x-1 {change_color}",
                    {change_icon}
                    span { class: "text-sm font-medium", "{change}" }
                }
            }
            div {
                h3 { class: "text-sm font-medium text-gray-500 uppercase tracking-wider", "{title}" }
                p { class: "text-2xl font-bold text-gray-900 mt-1", "{value}" }
            }
        }
    }
}

#[component]
fn ActionButton(icon: Element, label: &'static str) -> Element {
    rsx! {
        button { class: "flex flex-col items-center p-6 border-2 border-gray-200 rounded-xl hover:border-blue-300 hover:bg-blue-50 transition-colors group",
            div { class: "text-gray-600 group-hover:text-blue-600 transition-colors mb-2",
                {icon}
            }
            span { class: "text-sm font-medium text-gray-700 group-hover:text-blue-700", "{label}" }
        }
    }
}

/// Shared navbar component
#[component]
fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-white shadow-lg border-b",
            div { class: "max-w-7xl mx-auto px-4",
                div { class: "flex justify-between h-16",
                    div { class: "flex items-center space-x-8",
                        div { class: "flex items-center space-x-3",
                            lucide::Sparkles { class: "w-8 h-8 text-blue-600" }
                            span { class: "text-xl font-bold text-gray-800", "Lucide Dioxus" }
                        }
                        
                        div { class: "hidden md:flex space-x-4",
                            Link {
                                to: Route::Home {},
                                class: "flex items-center space-x-2 px-3 py-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors",
                                lucide::Home { class: "w-4 h-4" }
                                span { "Home" }
                            }
                            Link {
                                to: Route::Showcase {},
                                class: "flex items-center space-x-2 px-3 py-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors",
                                lucide::Grid3X3 { class: "w-4 h-4" }
                                span { "Showcase" }
                            }
                            Link {
                                to: Route::Dashboard {},
                                class: "flex items-center space-x-2 px-3 py-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors",
                                lucide::BarChart3 { class: "w-4 h-4" }
                                span { "Dashboard" }
                            }
                        }
                    }
                    
                    div { class: "flex items-center space-x-4",
                        button { class: "p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-md",
                            lucide::Github { class: "w-5 h-5" }
                        }
                        button { class: "p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-md",
                            lucide::ExternalLink { class: "w-5 h-5" }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
