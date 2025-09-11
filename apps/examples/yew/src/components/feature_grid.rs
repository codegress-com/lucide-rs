use yew::prelude::*;

#[component]
pub fn FeatureGrid() -> Html {
    html! {
        <div class="bg-white rounded-lg shadow-md border border-gray-200 p-6">
            <div class="flex items-center space-x-3 mb-6">
                <lucide::Grid3X3 class="w-6 h-6 text-blue-600" />
                <h2 class="text-xl font-bold text-gray-900">{"Features & Services"}</h2>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <FeatureCard 
                    icon={html!{<lucide::Shield class="w-12 h-12 text-green-600" />}}
                    title="Security"
                    description="Advanced security features to protect your data and privacy"
                />
                <FeatureCard 
                    icon={html!{<lucide::Zap class="w-12 h-12 text-yellow-600" />}}
                    title="Fast Performance"
                    description="Lightning-fast load times and optimized performance"
                />
                <FeatureCard 
                    icon={html!{<lucide::Smartphone class="w-12 h-12 text-purple-600" />}}
                    title="Mobile Ready"
                    description="Fully responsive design that works on all devices"
                />
                <FeatureCard 
                    icon={html!{<lucide::Cloud class="w-12 h-12 text-blue-600" />}}
                    title="Cloud Storage"
                    description="Secure cloud storage with automatic backups"
                />
                <FeatureCard 
                    icon={html!{<lucide::Headphones class="w-12 h-12 text-orange-600" />}}
                    title="24/7 Support"
                    description="Round-the-clock customer support and assistance"
                />
                <FeatureCard 
                    icon={html!{<lucide::Palette class="w-12 h-12 text-pink-600" />}}
                    title="Customizable"
                    description="Highly customizable themes and layouts"
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FeatureCardProps {
    pub icon: Html,
    pub title: String,
    pub description: String,
}

#[component]
pub fn FeatureCard(props: &FeatureCardProps) -> Html {
    html! {
        <div class="text-center p-6 hover:bg-gray-50 rounded-lg transition-colors">
            <div class="flex justify-center mb-4">
                {props.icon.clone()}
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">{&props.title}</h3>
            <p class="text-gray-600 text-sm leading-relaxed">{&props.description}</p>
        </div>
    }
}
