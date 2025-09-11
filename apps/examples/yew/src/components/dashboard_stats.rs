use yew::prelude::*;

#[component]
pub fn DashboardStats() -> Html {
    html! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
            <StatCard 
                icon={html!{<lucide::Users class="w-8 h-8 text-blue-600" />}}
                title="Total Users"
                value="12,345"
                change="+12%"
                change_positive={true}
            />
            <StatCard 
                icon={html!{<lucide::ShoppingCart class="w-8 h-8 text-green-600" />}}
                title="Sales"
                value="$54,321"
                change="+8%"
                change_positive={true}
            />
            <StatCard 
                icon={html!{<lucide::TrendingUp class="w-8 h-8 text-purple-600" />}}
                title="Growth"
                value="23.5%"
                change="+2.1%"
                change_positive={true}
            />
            <StatCard 
                icon={html!{<lucide::Activity class="w-8 h-8 text-orange-600" />}}
                title="Activity"
                value="89%"
                change="-1.2%"
                change_positive={false}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct StatCardProps {
    pub icon: Html,
    pub title: String,
    pub value: String,
    pub change: String,
    pub change_positive: bool,
}

#[component]
pub fn StatCard(props: &StatCardProps) -> Html {
    let change_color = if props.change_positive {
        "text-green-600"
    } else {
        "text-red-600"
    };
    
    let change_icon = if props.change_positive {
        html!{<lucide::ArrowUp class="w-4 h-4" />}
    } else {
        html!{<lucide::ArrowDown class="w-4 h-4" />}
    };

    html! {
        <div class="bg-white p-6 rounded-lg shadow-md border border-gray-200">
            <div class="flex items-center justify-between mb-4">
                <div class="p-2 bg-gray-50 rounded-lg">
                    {props.icon.clone()}
                </div>
                <div class={format!("flex items-center space-x-1 {}", change_color)}>
                    {change_icon}
                    <span class="text-sm font-medium">{&props.change}</span>
                </div>
            </div>
            <div>
                <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wider">{&props.title}</h3>
                <p class="text-2xl font-bold text-gray-900 mt-1">{&props.value}</p>
            </div>
        </div>
    }
}
