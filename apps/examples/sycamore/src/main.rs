use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        view! {
            // Add Tailwind CSS
            link(rel="stylesheet", href="https://cdn.tailwindcss.com")
            
            div(class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100") {
                // Header
                header(class="bg-white shadow-lg border-b") {
                    div(class="max-w-6xl mx-auto px-4 py-4") {
                        div(class="flex items-center justify-between") {
                            div(class="flex items-center space-x-3") {
                                lucide::Sparkles(class="w-10 h-10 text-purple-600")
                                div {
                                    h1(class="text-2xl font-bold text-gray-800") { "Lucide Sycamore" }
                                    p(class="text-sm text-gray-600") { "Icons for Sycamore apps" }
                                }
                            }
                            
                            div(class="flex items-center space-x-4") {
                                button(class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-lg transition-colors") {
                                    lucide::Github(class="w-6 h-6")
                                }
                                button(class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-lg transition-colors") {
                                    lucide::ExternalLink(class="w-6 h-6")
                                }
                            }
                        }
                    }
                }
                
                // Main Content
                main(class="max-w-6xl mx-auto px-4 py-12") {
                    // Hero Section
                    section(class="text-center py-16") {
                        div(class="flex justify-center mb-8") {
                            lucide::Zap(class="w-24 h-24 text-yellow-500 drop-shadow-lg")
                        }
                        h1(class="text-5xl font-bold text-gray-900 mb-6") {
                            "Beautiful Icons for "
                            span(class="text-purple-600") { "Sycamore" }
                        }
                        p(class="text-xl text-gray-600 max-w-3xl mx-auto mb-10") {
                            "Enhance your Sycamore applications with our comprehensive collection of "
                            "SVG icons. Lightweight, customizable, and easy to use."
                        }
                        
                        div(class="flex justify-center space-x-4") {
                            button(class="flex items-center space-x-2 px-8 py-4 bg-purple-600 text-white rounded-xl hover:bg-purple-700 transition-colors shadow-lg") {
                                lucide::Download(class="w-5 h-5")
                                span(class="font-semibold") { "Get Started" }
                            }
                            button(class="flex items-center space-x-2 px-8 py-4 border-2 border-gray-300 text-gray-700 rounded-xl hover:bg-white transition-colors") {
                                lucide::BookOpen(class="w-5 h-5")
                                span(class="font-semibold") { "Documentation" }
                            }
                        }
                    }
                    
                    // Features Section
                    section(class="py-16") {
                        div(class="text-center mb-12") {
                            h2(class="text-3xl font-bold text-gray-900 mb-4") { "Why Choose Lucide?" }
                            p(class="text-gray-600") { "Perfect for modern Sycamore applications" }
                        }
                        
                        div(class="grid grid-cols-1 md:grid-cols-3 gap-8") {
                            // Feature 1
                            div(class="text-center p-8 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-shadow") {
                                div(class="flex justify-center mb-6") {
                                    lucide::Gauge(class="w-12 h-12 text-blue-600")
                                }
                                h3(class="text-xl font-semibold text-gray-900 mb-4") { "High Performance" }
                                p(class="text-gray-600") { "Optimized SVGs with minimal bundle size impact" }
                            }
                            
                            // Feature 2
                            div(class="text-center p-8 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-shadow") {
                                div(class="flex justify-center mb-6") {
                                    lucide::Palette(class="w-12 h-12 text-purple-600")
                                }
                                h3(class="text-xl font-semibold text-gray-900 mb-4") { "Customizable" }
                                p(class="text-gray-600") { "Easy to style with classes and CSS variables" }
                            }
                            
                            // Feature 3
                            div(class="text-center p-8 bg-white rounded-2xl shadow-lg hover:shadow-xl transition-shadow") {
                                div(class="flex justify-center mb-6") {
                                    lucide::TreePine(class="w-12 h-12 text-green-600")
                                }
                                h3(class="text-xl font-semibold text-gray-900 mb-4") { "Tree Shakeable" }
                                p(class="text-gray-600") { "Only include the icons you actually use" }
                            }
                        }
                    }
                    
                    // Icon Showcase Section
                    section(class="py-16") {
                        div(class="bg-white rounded-2xl shadow-lg p-8") {
                            div(class="text-center mb-8") {
                                h2(class="text-3xl font-bold text-gray-900 mb-4") { "Icon Categories" }
                                p(class="text-gray-600") { "A sample of our comprehensive icon library" }
                            }
                            
                            // Navigation Icons
                            div(class="mb-12") {
                                h3(class="text-xl font-semibold text-gray-800 mb-6") { "Navigation & UI" }
                                div(class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-6") {
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Home(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Home" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Menu(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Menu" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Search(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Search" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Settings(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Settings" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::ArrowRight(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Arrow" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::ChevronDown(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Chevron" }
                                    }
                                }
                            }
                            
                            // Communication Icons
                            div(class="mb-12") {
                                h3(class="text-xl font-semibold text-gray-800 mb-6") { "Communication" }
                                div(class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-6") {
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Mail(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Mail" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::MessageCircle(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Message" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Phone(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Phone" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Send(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Send" }
                                    }
                                }
                            }
                            
                            // Files Icons
                            div {
                                h3(class="text-xl font-semibold text-gray-800 mb-6") { "Files & Documents" }
                                div(class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-6") {
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::File(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "File" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Folder(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Folder" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Download(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Download" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Upload(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Upload" }
                                    }
                                    div(class="flex flex-col items-center p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group") {
                                        div(class="text-gray-700 group-hover:text-purple-600 transition-colors mb-2") {
                                            lucide::Share(class="w-8 h-8")
                                        }
                                        span(class="text-xs text-gray-600 font-medium") { "Share" }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Footer
                    footer(class="text-center py-12 border-t border-gray-200 mt-16") {
                        div(class="flex justify-center items-center space-x-2 text-gray-600") {
                            span { "Built with" }
                            lucide::Heart(class="w-4 h-4 text-red-500")
                            span { "using Sycamore and Lucide" }
                        }
                    }
                }
            }
        }
    });
}
