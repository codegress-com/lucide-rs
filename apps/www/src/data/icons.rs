#[derive(Debug, Clone)]
pub struct IconInfo {
    pub name: &'static str,
    pub display_name: &'static str,
    pub category: &'static str,
    pub tags: &'static [&'static str],
    pub svg_path: &'static str,
}

// Sample Lucide icons data
pub const ICONS: &[IconInfo] = &[
    // Navigation & UI
    IconInfo {
        name: "home",
        display_name: "Home",
        category: "navigation",
        tags: &["house", "building", "residence"],
        svg_path: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z M9 22V12h6v10"
    },
    IconInfo {
        name: "search",
        display_name: "Search",
        category: "navigation",
        tags: &["find", "magnify", "look"],
        svg_path: "m21 21-6-6m2-5a7 7 0 1 1-14 0 7 7 0 0 1 14 0z"
    },
    IconInfo {
        name: "menu",
        display_name: "Menu",
        category: "navigation",
        tags: &["hamburger", "bars", "navigation"],
        svg_path: "M3 6h18 M3 12h18 M3 18h18"
    },
    IconInfo {
        name: "x",
        display_name: "X",
        category: "navigation",
        tags: &["close", "cancel", "cross"],
        svg_path: "M18 6 6 18 M6 6l12 12"
    },
    IconInfo {
        name: "arrow-left",
        display_name: "Arrow Left",
        category: "navigation",
        tags: &["back", "previous", "return"],
        svg_path: "m12 19-7-7 7-7 M5 12h14"
    },
    IconInfo {
        name: "arrow-right",
        display_name: "Arrow Right",
        category: "navigation",
        tags: &["forward", "next", "continue"],
        svg_path: "M5 12h14 m-7-7 7 7-7 7"
    },

    // User & Profile
    IconInfo {
        name: "user",
        display_name: "User",
        category: "user",
        tags: &["person", "profile", "account"],
        svg_path: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2 M12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8z"
    },
    IconInfo {
        name: "users",
        display_name: "Users",
        category: "user",
        tags: &["people", "group", "team"],
        svg_path: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2 M9 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8z M22 21v-2a4 4 0 0 0-3-3.87 M16 3.13a4 4 0 0 1 0 7.75"
    },
    IconInfo {
        name: "user-plus",
        display_name: "User Plus",
        category: "user",
        tags: &["add", "person", "invite"],
        svg_path: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2 M9 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8z M22 9h-6 M19 6v6"
    },

    // Actions
    IconInfo {
        name: "heart",
        display_name: "Heart",
        category: "actions",
        tags: &["like", "love", "favorite"],
        svg_path: "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"
    },
    IconInfo {
        name: "star",
        display_name: "Star",
        category: "actions",
        tags: &["favorite", "rating", "bookmark"],
        svg_path: "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"
    },
    IconInfo {
        name: "thumbs-up",
        display_name: "Thumbs Up",
        category: "actions",
        tags: &["like", "approve", "good"],
        svg_path: "M7 10v12 M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h3.5a2 2 0 0 1 2 2.06l-.44 2.94"
    },
    IconInfo {
        name: "share",
        display_name: "Share",
        category: "actions",
        tags: &["send", "distribute", "export"],
        svg_path: "M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8 M16 6l-4-4-4 4 M12 2v13"
    },

    // Media & Files
    IconInfo {
        name: "image",
        display_name: "Image",
        category: "media",
        tags: &["picture", "photo", "gallery"],
        svg_path: "M5 2h14a3 3 0 0 1 3 3v14a3 3 0 0 1-3 3H5a3 3 0 0 1-3-3V5a3 3 0 0 1 3-3z M9 9a2 2 0 1 0 0-4 2 2 0 0 0 0 4z m13 13-5-5-5 5"
    },
    IconInfo {
        name: "file",
        display_name: "File",
        category: "media",
        tags: &["document", "page", "text"],
        svg_path: "M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z M13 2v7h7"
    },
    IconInfo {
        name: "download",
        display_name: "Download",
        category: "media",
        tags: &["save", "get", "import"],
        svg_path: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M7 10l5 5 5-5 M12 15V3"
    },
    IconInfo {
        name: "upload",
        display_name: "Upload",
        category: "media",
        tags: &["send", "put", "export"],
        svg_path: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4 M17 8l-5-5-5 5 M12 3v12"
    },

    // Communication
    IconInfo {
        name: "mail",
        display_name: "Mail",
        category: "communication",
        tags: &["email", "message", "envelope"],
        svg_path: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z M22 6l-10 7L2 6"
    },
    IconInfo {
        name: "message-circle",
        display_name: "Message Circle",
        category: "communication",
        tags: &["chat", "talk", "bubble"],
        svg_path: "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"
    },
    IconInfo {
        name: "phone",
        display_name: "Phone",
        category: "communication",
        tags: &["call", "telephone", "contact"],
        svg_path: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"
    },

    // System & Settings
    IconInfo {
        name: "settings",
        display_name: "Settings",
        category: "system",
        tags: &["gear", "preferences", "config"],
        svg_path: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"
    },
    IconInfo {
        name: "trash",
        display_name: "Trash",
        category: "system",
        tags: &["delete", "remove", "bin"],
        svg_path: "M3 6h18 M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
    },
    IconInfo {
        name: "edit",
        display_name: "Edit",
        category: "system",
        tags: &["pencil", "modify", "write"],
        svg_path: "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7 M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
    },
    IconInfo {
        name: "copy",
        display_name: "Copy",
        category: "system",
        tags: &["duplicate", "clone", "clipboard"],
        svg_path: "M20 9h-9V5a2 2 0 0 1 2-2h5a2 2 0 0 1 2 2v4z M4 3h9v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z M12 19a2 2 0 0 1-2-2V9h7a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-5z"
    },

    // Business & Commerce
    IconInfo {
        name: "shopping-cart",
        display_name: "Shopping Cart",
        category: "commerce",
        tags: &["buy", "purchase", "basket"],
        svg_path: "M9 22a1 1 0 1 0 0-2 1 1 0 0 0 0 2z M20 22a1 1 0 1 0 0-2 1 1 0 0 0 0 2z M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"
    },
    IconInfo {
        name: "credit-card",
        display_name: "Credit Card",
        category: "commerce",
        tags: &["payment", "money", "finance"],
        svg_path: "M1 4h22v16H1V4z M1 10h22"
    },
    IconInfo {
        name: "dollar-sign",
        display_name: "Dollar Sign",
        category: "commerce",
        tags: &["money", "currency", "price"],
        svg_path: "M12 1v22 M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"
    },

    // Development & Tech
    IconInfo {
        name: "code",
        display_name: "Code",
        category: "development",
        tags: &["programming", "developer", "script"],
        svg_path: "m16 18 6-6-6-6 M8 6l-6 6 6 6"
    },
    IconInfo {
        name: "terminal",
        display_name: "Terminal",
        category: "development",
        tags: &["console", "command", "shell"],
        svg_path: "M4 17l6-6-6-6 M12 19h8"
    },
    IconInfo {
        name: "github",
        display_name: "GitHub",
        category: "development",
        tags: &["git", "repository", "version"],
        svg_path: "M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4 M9 18c-4.51 2-5-2-7-2"
    },
    IconInfo {
        name: "database",
        display_name: "Database",
        category: "development",
        tags: &["storage", "data", "server"],
        svg_path: "M12 8c-6.627 0-12-1.79-12-4s5.373-4 12-4 12 1.79 12 4-5.373 4-12 4z M0 4v4c0 2.21 5.373 4 12 4s12-1.79 12-4V4 M0 12v4c0 2.21 5.373 4 12 4s12-1.79 12-4v-4"
    },

    // Time & Calendar
    IconInfo {
        name: "calendar",
        display_name: "Calendar",
        category: "time",
        tags: &["date", "schedule", "event"],
        svg_path: "M3 4h18a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z M16 2v4 M8 2v4 M3 10h18"
    },
    IconInfo {
        name: "clock",
        display_name: "Clock",
        category: "time",
        tags: &["time", "watch", "schedule"],
        svg_path: "M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z M12 6v6l4 2"
    },

    // Weather & Nature
    IconInfo {
        name: "sun",
        display_name: "Sun",
        category: "weather",
        tags: &["light", "day", "bright"],
        svg_path: "M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8z M12 1v2 M12 21v2 M4.22 4.22l1.42 1.42 M18.36 18.36l1.42 1.42 M1 12h2 M21 12h2 M4.22 19.78l1.42-1.42 M18.36 5.64l1.42-1.42"
    },
    IconInfo {
        name: "moon",
        display_name: "Moon",
        category: "weather",
        tags: &["night", "dark", "sleep"],
        svg_path: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
    },
    IconInfo {
        name: "cloud",
        display_name: "Cloud",
        category: "weather",
        tags: &["sky", "storage", "weather"],
        svg_path: "M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z"
    },
];

pub fn get_icons_by_category(category: &str) -> Vec<&'static IconInfo> {
    ICONS.iter().filter(|icon| icon.category == category).collect()
}

pub fn search_icons(query: &str) -> Vec<&'static IconInfo> {
    let query = query.to_lowercase();
    if query.is_empty() {
        return ICONS.iter().collect();
    }
    
    ICONS.iter().filter(|icon| {
        icon.name.contains(&query) ||
        icon.display_name.to_lowercase().contains(&query) ||
        icon.tags.iter().any(|tag| tag.contains(&query))
    }).collect()
}

pub fn get_categories() -> Vec<&'static str> {
    let mut categories: Vec<&str> = ICONS.iter().map(|icon| icon.category).collect();
    categories.sort();
    categories.dedup();
    categories
}
