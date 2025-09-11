use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-6xl mx-auto">
            // Hero Section
            <div class="text-center py-16">
                <div class="flex justify-center mb-6">
                    <lucide::Zap class="w-20 h-20 text-yellow-500" />
                </div>
                <h1 class="text-5xl font-bold text-gray-900 mb-6">
                    "Lucide Icons for Leptos"
                </h1>
                <p class="text-xl text-gray-600 max-w-3xl mx-auto mb-8">
                    "Beautiful, customizable SVG icons for modern Leptos applications. "
                    "Lightning fast, tree-shakeable, and developer friendly."
                </p>
                
                <div class="flex justify-center space-x-4 mb-12">
                    <button class="flex items-center space-x-2 px-8 py-4 bg-blue-600 text-white rounded-xl hover:bg-blue-700 transition-colors shadow-lg">
                        <lucide::Rocket class="w-6 h-6" />
                        <span class="font-semibold">"Get Started"</span>
                    </button>
                    <button class="flex items-center space-x-2 px-8 py-4 border-2 border-gray-300 text-gray-700 rounded-xl hover:bg-gray-50 transition-colors">
                        <lucide::Code class="w-6 h-6" />
                        <span class="font-semibold">"View Docs"</span>
                    </button>
                </div>
            </div>
            
            // Quick Stats
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-16">
                <div class="text-center p-8 bg-white rounded-2xl shadow-lg">
                    <lucide::Package class="w-12 h-12 text-blue-600 mx-auto mb-4" />
                    <h3 class="text-2xl font-bold text-gray-900">"1000+"</h3>
                    <p class="text-gray-600">"Icons Available"</p>
                </div>
                <div class="text-center p-8 bg-white rounded-2xl shadow-lg">
                    <lucide::Feather class="w-12 h-12 text-green-600 mx-auto mb-4" />
                    <h3 class="text-2xl font-bold text-gray-900">"Ultra Light"</h3>
                    <p class="text-gray-600">"Optimized Bundle Size"</p>
                </div>
                <div class="text-center p-8 bg-white rounded-2xl shadow-lg">
                    <lucide::Palette class="w-12 h-12 text-purple-600 mx-auto mb-4" />
                    <h3 class="text-2xl font-bold text-gray-900">"Customizable"</h3>
                    <p class="text-gray-600">"Colors, Sizes & Strokes"</p>
                </div>
            </div>
            
            // Feature showcase
            <div class="bg-white rounded-2xl shadow-lg p-8">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900 mb-4">"Why Choose Lucide?"</h2>
                    <p class="text-gray-600">"Perfect for modern web applications"</p>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <FeatureCard 
                        icon=view!{<lucide::Gauge class="w-8 h-8 text-blue-600" />}
                        title="High Performance"
                        description="Optimized SVGs with minimal overhead"
                    />
                    <FeatureCard 
                        icon=view!{<lucide::TreePine class="w-8 h-8 text-green-600" />}
                        title="Tree Shakeable"
                        description="Only include the icons you actually use"
                    />
                    <FeatureCard 
                        icon=view!{<lucide::Paintbrush class="w-8 h-8 text-purple-600" />}
                        title="Fully Customizable"
                        description="Change colors, sizes, and stroke width easily"
                    />
                    <FeatureCard 
                        icon=view!{<lucide::Smartphone class="w-8 h-8 text-orange-600" />}
                        title="Responsive"
                        description="Works perfectly on all screen sizes"
                    />
                    <FeatureCard 
                        icon=view!{<lucide::Github class="w-8 h-8 text-gray-800" />}
                        title="Open Source"
                        description="Free to use with MIT license"
                    />
                    <FeatureCard 
                        icon=view!{<lucide::Heart class="w-8 h-8 text-red-600" />}
                        title="Developer Friendly"
                        description="Great DX with TypeScript support"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn FeatureCard(
    icon: View,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="text-center p-6 hover:bg-gray-50 rounded-xl transition-colors">
            <div class="flex justify-center mb-4">{icon}</div>
            <h3 class="text-xl font-semibold text-gray-900 mb-3">{title}</h3>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}

#[component]
pub fn IconsPage() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto space-y-12">
            // Header
            <div class="text-center">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">"Icon Library"</h1>
                <p class="text-xl text-gray-600">"Explore our comprehensive collection of icons"</p>
            </div>
            
            // Search bar
            <div class="flex justify-center">
                <div class="relative w-full max-w-md">
                    <lucide::Search class="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" />
                    <input 
                        type="text" 
                        placeholder="Search icons..." 
                        class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    />
                </div>
            </div>
            
            // Icon categories
            <IconCategory 
                title="Navigation & UI"
                icons=vec![
                    ("Home", view!{<lucide::Home class="w-8 h-8" />}),
                    ("Menu", view!{<lucide::Menu class="w-8 h-8" />}),
                    ("Search", view!{<lucide::Search class="w-8 h-8" />}),
                    ("Settings", view!{<lucide::Settings class="w-8 h-8" />}),
                    ("User", view!{<lucide::User class="w-8 h-8" />}),
                    ("Bell", view!{<lucide::Bell class="w-8 h-8" />}),
                    ("Arrow Right", view!{<lucide::ArrowRight class="w-8 h-8" />}),
                    ("Chevron Down", view!{<lucide::ChevronDown class="w-8 h-8" />}),
                ]
            />
            
            <IconCategory 
                title="Communication"
                icons=vec![
                    ("Mail", view!{<lucide::Mail class="w-8 h-8" />}),
                    ("Message Circle", view!{<lucide::MessageCircle class="w-8 h-8" />}),
                    ("Phone", view!{<lucide::Phone class="w-8 h-8" />}),
                    ("Video", view!{<lucide::Video class="w-8 h-8" />}),
                    ("Send", view!{<lucide::Send class="w-8 h-8" />}),
                    ("At Sign", view!{<lucide::AtSign class="w-8 h-8" />}),
                    ("MessageSquare", view!{<lucide::MessageSquare class="w-8 h-8" />}),
                    ("Mic", view!{<lucide::Mic class="w-8 h-8" />}),
                ]
            />
            
            <IconCategory 
                title="Files & Storage"
                icons=vec![
                    ("File", view!{<lucide::File class="w-8 h-8" />}),
                    ("Folder", view!{<lucide::Folder class="w-8 h-8" />}),
                    ("Download", view!{<lucide::Download class="w-8 h-8" />}),
                    ("Upload", view!{<lucide::Upload class="w-8 h-8" />}),
                    ("Save", view!{<lucide::Save class="w-8 h-8" />}),
                    ("Share", view!{<lucide::Share class="w-8 h-8" />}),
                    ("Hard Drive", view!{<lucide::HardDrive class="w-8 h-8" />}),
                    ("Database", view!{<lucide::Database class="w-8 h-8" />}),
                ]
            />
            
            <IconCategory 
                title="Media & Entertainment"
                icons=vec![
                    ("Play", view!{<lucide::Play class="w-8 h-8" />}),
                    ("Pause", view!{<lucide::Pause class="w-8 h-8" />}),
                    ("Volume 2", view!{<lucide::Volume2 class="w-8 h-8" />}),
                    ("Camera", view!{<lucide::Camera class="w-8 h-8" />}),
                    ("Image", view!{<lucide::Image class="w-8 h-8" />}),
                    ("Music", view!{<lucide::Music class="w-8 h-8" />}),
                    ("Film", view!{<lucide::Film class="w-8 h-8" />}),
                    ("Headphones", view!{<lucide::Headphones class="w-8 h-8" />}),
                ]
            />
        </div>
    }
}

#[component]
fn IconCategory(
    title: &'static str,
    icons: Vec<(&'static str, View)>,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-2xl shadow-lg p-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-6">{title}</h2>
            <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-6">
                {icons.into_iter().map(|(name, icon)| view! {
                    <div class="flex flex-col items-center space-y-3 p-4 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer group">
                        <div class="text-gray-700 group-hover:text-blue-600 transition-colors">
                            {icon}
                        </div>
                        <span class="text-xs text-gray-600 text-center font-medium">{name}</span>
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}

