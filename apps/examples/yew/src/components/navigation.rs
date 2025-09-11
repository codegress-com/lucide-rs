use yew::prelude::*;

#[component]
pub fn Navigation() -> Html {
    html! {
        <nav class="bg-white shadow-lg border-b">
            <div class="max-w-7xl mx-auto px-4">
                <div class="flex justify-between h-16">
                    <div class="flex items-center space-x-8">
                        <div class="flex items-center space-x-2">
                            <lucide::Sparkles class="w-8 h-8 text-blue-600" />
                            <span class="text-xl font-bold text-gray-800">{"Lucide Icons"}</span>
                        </div>
                        
                        <div class="hidden md:flex space-x-4">
                            <NavItem href="#dashboard" icon={html!{<lucide::LayoutDashboard class="w-4 h-4" />}} text="Dashboard" />
                            <NavItem href="#icons" icon={html!{<lucide::Palette class="w-4 h-4" />}} text="Icons" />
                            <NavItem href="#components" icon={html!{<lucide::Package class="w-4 h-4" />}} text="Components" />
                            <NavItem href="#docs" icon={html!{<lucide::BookOpen class="w-4 h-4" />}} text="Docs" />
                        </div>
                    </div>
                    
                    <div class="flex items-center space-x-4">
                        <button class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-md">
                            <lucide::Search class="w-5 h-5" />
                        </button>
                        <button class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-md">
                            <lucide::Bell class="w-5 h-5" />
                        </button>
                        <button class="flex items-center space-x-2 p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-md">
                            <lucide::User class="w-5 h-5" />
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavItemProps {
    pub href: String,
    pub icon: Html,
    pub text: String,
}

#[component]
pub fn NavItem(props: &NavItemProps) -> Html {
    html! {
        <a href={props.href.clone()} class="flex items-center space-x-2 px-3 py-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
            {props.icon.clone()}
            <span>{&props.text}</span>
        </a>
    }
}
