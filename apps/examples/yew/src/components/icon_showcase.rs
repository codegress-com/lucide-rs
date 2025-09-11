use yew::prelude::*;

#[component]
pub fn IconShowcase() -> Html {
    html! {
        <div class="bg-white rounded-lg shadow-md border border-gray-200 p-6">
            <div class="flex items-center space-x-3 mb-6">
                <lucide::Sparkles class="w-6 h-6 text-blue-600" />
                <h2 class="text-xl font-bold text-gray-900">{"Icon Categories"}</h2>
            </div>
            
            <div class="space-y-8">
                <IconCategory 
                    title="Navigation & UI"
                    icons={vec![
                        (html!{<lucide::Home class="w-6 h-6" />}, "Home"),
                        (html!{<lucide::Menu class="w-6 h-6" />}, "Menu"),
                        (html!{<lucide::Search class="w-6 h-6" />}, "Search"),
                        (html!{<lucide::Settings class="w-6 h-6" />}, "Settings"),
                        (html!{<lucide::ChevronDown class="w-6 h-6" />}, "Chevron Down"),
                        (html!{<lucide::X class="w-6 h-6" />}, "Close"),
                    ]}
                />
                
                <IconCategory 
                    title="Communication"
                    icons={vec![
                        (html!{<lucide::Mail class="w-6 h-6" />}, "Mail"),
                        (html!{<lucide::MessageCircle class="w-6 h-6" />}, "Message"),
                        (html!{<lucide::Phone class="w-6 h-6" />}, "Phone"),
                        (html!{<lucide::Video class="w-6 h-6" />}, "Video"),
                        (html!{<lucide::Send class="w-6 h-6" />}, "Send"),
                        (html!{<lucide::Bell class="w-6 h-6" />}, "Bell"),
                    ]}
                />
                
                <IconCategory 
                    title="Files & Documents"
                    icons={vec![
                        (html!{<lucide::File class="w-6 h-6" />}, "File"),
                        (html!{<lucide::Folder class="w-6 h-6" />}, "Folder"),
                        (html!{<lucide::Download class="w-6 h-6" />}, "Download"),
                        (html!{<lucide::Upload class="w-6 h-6" />}, "Upload"),
                        (html!{<lucide::Save class="w-6 h-6" />}, "Save"),
                        (html!{<lucide::Share class="w-6 h-6" />}, "Share"),
                    ]}
                />
                
                <IconCategory 
                    title="Media & Entertainment"
                    icons={vec![
                        (html!{<lucide::Play class="w-6 h-6" />}, "Play"),
                        (html!{<lucide::Pause class="w-6 h-6" />}, "Pause"),
                        (html!{<lucide::Volume2 class="w-6 h-6" />}, "Volume"),
                        (html!{<lucide::Camera class="w-6 h-6" />}, "Camera"),
                        (html!{<lucide::Image class="w-6 h-6" />}, "Image"),
                        (html!{<lucide::Music class="w-6 h-6" />}, "Music"),
                    ]}
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconCategoryProps {
    pub title: String,
    pub icons: Vec<(Html, &'static str)>,
}

#[component]
pub fn IconCategory(props: &IconCategoryProps) -> Html {
    html! {
        <div>
            <h3 class="text-lg font-semibold text-gray-800 mb-4">{&props.title}</h3>
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-4">
                {for props.icons.iter().map(|(icon, name)| {
                    html! {
                        <div class="flex flex-col items-center p-3 hover:bg-gray-50 rounded-lg transition-colors cursor-pointer">
                            <div class="text-gray-700 hover:text-blue-600 transition-colors mb-2">
                                {icon.clone()}
                            </div>
                            <span class="text-xs text-gray-600 text-center">{name}</span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
