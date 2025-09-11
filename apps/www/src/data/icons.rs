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
    
    // More Navigation Icons
    IconInfo {
        name: "chevron-up",
        display_name: "Chevron Up",
        category: "navigation",
        tags: &["arrow", "up", "direction"],
        svg_path: "m18 15-6-6-6 6"
    },
    IconInfo {
        name: "chevron-down",
        display_name: "Chevron Down",
        category: "navigation",
        tags: &["arrow", "down", "direction"],
        svg_path: "m6 9 6 6 6-6"
    },
    IconInfo {
        name: "chevron-left",
        display_name: "Chevron Left",
        category: "navigation",
        tags: &["arrow", "left", "direction"],
        svg_path: "m15 18-6-6 6-6"
    },
    IconInfo {
        name: "chevron-right",
        display_name: "Chevron Right",
        category: "navigation",
        tags: &["arrow", "right", "direction"],
        svg_path: "m9 18 6-6-6-6"
    },
    IconInfo {
        name: "plus",
        display_name: "Plus",
        category: "navigation",
        tags: &["add", "create", "new"],
        svg_path: "M5 12h14 M12 5v14"
    },
    IconInfo {
        name: "minus",
        display_name: "Minus",
        category: "navigation",
        tags: &["remove", "subtract", "delete"],
        svg_path: "M5 12h14"
    },
    IconInfo {
        name: "more-horizontal",
        display_name: "More Horizontal",
        category: "navigation",
        tags: &["dots", "menu", "options"],
        svg_path: "M5 12h.01 M12 12h.01 M19 12h.01"
    },
    IconInfo {
        name: "more-vertical",
        display_name: "More Vertical",
        category: "navigation",
        tags: &["dots", "menu", "options"],
        svg_path: "M12 5h.01 M12 12h.01 M12 19h.01"
    },
    
    // Additional Actions
    IconInfo {
        name: "bookmark",
        display_name: "Bookmark",
        category: "actions",
        tags: &["save", "favorite", "mark"],
        svg_path: "m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"
    },
    IconInfo {
        name: "eye",
        display_name: "Eye",
        category: "actions",
        tags: &["view", "see", "watch"],
        svg_path: "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z M12 9a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"
    },
    IconInfo {
        name: "eye-off",
        display_name: "Eye Off",
        category: "actions",
        tags: &["hide", "invisible", "private"],
        svg_path: "M9.88 9.88a3 3 0 1 0 4.24 4.24 m-1.8-7.07c-1.32-.5-2.75-.73-4.32-.73C2 6.32 2 6.32 2 12s0 5.68 0 5.68c0 0 3 7 10 7a13.16 13.16 0 0 0 5.82-1.35M22 2 2 22"
    },
    IconInfo {
        name: "lock",
        display_name: "Lock",
        category: "actions",
        tags: &["secure", "private", "password"],
        svg_path: "M11 3a4 4 0 0 1 8 0v6H7V9a4 4 0 0 1 4-4Zm8 8v8a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2Z"
    },
    IconInfo {
        name: "unlock",
        display_name: "Unlock",
        category: "actions",
        tags: &["open", "unlocked", "access"],
        svg_path: "M7 11V7a5 5 0 0 1 9.9-1M7 11h10v8a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2v-8Z"
    },
    
    // Technology & Devices
    IconInfo {
        name: "smartphone",
        display_name: "Smartphone",
        category: "devices",
        tags: &["mobile", "phone", "device"],
        svg_path: "M7 2h10a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2zm5 15h.01"
    },
    IconInfo {
        name: "laptop",
        display_name: "Laptop",
        category: "devices",
        tags: &["computer", "pc", "work"],
        svg_path: "M20 16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v9m16 0H4m16 0 1.28 2.55a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45L4 16"
    },
    IconInfo {
        name: "monitor",
        display_name: "Monitor",
        category: "devices",
        tags: &["screen", "display", "computer"],
        svg_path: "M20 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2ZM8 21h8m-4-4v4"
    },
    IconInfo {
        name: "tablet",
        display_name: "Tablet",
        category: "devices",
        tags: &["ipad", "device", "touch"],
        svg_path: "M19 2H5a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2Z"
    },
    
    // Social & Communication
    IconInfo {
        name: "wifi",
        display_name: "Wifi",
        category: "communication",
        tags: &["internet", "connection", "wireless"],
        svg_path: "m1 9 2 3c7.5-7.5 19-7.5 26 0l2-3c-8.5-8.5-21.5-8.5-30 0Z m6 6 2 3c3.5-3.5 8.5-3.5 12 0l2-3c-5-5-13-5-18 0Z m6 6h.01"
    },
    IconInfo {
        name: "bluetooth",
        display_name: "Bluetooth",
        category: "communication",
        tags: &["wireless", "connection", "device"],
        svg_path: "m7 7 10 5-5 5V2l5 5L7 17"
    },
    
    // Text & Content
    IconInfo {
        name: "type",
        display_name: "Type",
        category: "text",
        tags: &["font", "typography", "text"],
        svg_path: "M4 7V4h16v3M9 20h6M12 4v16"
    },
    IconInfo {
        name: "bold",
        display_name: "Bold",
        category: "text",
        tags: &["format", "strong", "weight"],
        svg_path: "M14 12a4 4 0 0 0 0-8H6v8h8Zm0 0a4 4 0 0 1 0 8H6v-8h8Z"
    },
    IconInfo {
        name: "italic",
        display_name: "Italic",
        category: "text",
        tags: &["format", "slant", "style"],
        svg_path: "M19 4h-9m4 16H5m4-16L7 20"
    },
    IconInfo {
        name: "underline",
        display_name: "Underline",
        category: "text",
        tags: &["format", "text", "decoration"],
        svg_path: "M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3M4 21h16"
    },
    
    // Layout & Design
    IconInfo {
        name: "align-left",
        display_name: "Align Left",
        category: "layout",
        tags: &["text", "format", "alignment"],
        svg_path: "M21 6H3M15 12H3M17 18H3"
    },
    IconInfo {
        name: "align-center",
        display_name: "Align Center",
        category: "layout",
        tags: &["text", "format", "alignment"],
        svg_path: "M17 6H7M19 12H5M21 18H3"
    },
    IconInfo {
        name: "align-right",
        display_name: "Align Right",
        category: "layout",
        tags: &["text", "format", "alignment"],
        svg_path: "M21 6H9M21 12H15M21 18H7"
    },
    IconInfo {
        name: "grid",
        display_name: "Grid",
        category: "layout",
        tags: &["layout", "structure", "organize"],
        svg_path: "M3 3h7v7H3zM14 3h7v7h-7zM14 14h7v7h-7zM3 14h7v7H3z"
    },
    IconInfo {
        name: "columns",
        display_name: "Columns",
        category: "layout",
        tags: &["layout", "structure", "vertical"],
        svg_path: "M12 3h7a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1h-7m0-18H5a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h7m0-18v18"
    },
    
    // Status & Alerts
    IconInfo {
        name: "check",
        display_name: "Check",
        category: "status",
        tags: &["done", "complete", "success"],
        svg_path: "M20 6 9 17l-5-5"
    },
    IconInfo {
        name: "check-circle",
        display_name: "Check Circle",
        category: "status",
        tags: &["done", "complete", "success"],
        svg_path: "M22 11.08V12a10 10 0 1 1-5.93-9.14m-1.42 11.63L20 6"
    },
    IconInfo {
        name: "alert-circle",
        display_name: "Alert Circle",
        category: "status",
        tags: &["warning", "error", "caution"],
        svg_path: "M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10Z m0-6h.01 M12 8v4"
    },
    IconInfo {
        name: "alert-triangle",
        display_name: "Alert Triangle",
        category: "status",
        tags: &["warning", "error", "caution"],
        svg_path: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z M12 9v4 M12 17h.01"
    },
    IconInfo {
        name: "info",
        display_name: "Info",
        category: "status",
        tags: &["information", "help", "details"],
        svg_path: "M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10Z M12 16v-4 M12 8h.01"
    },
    
    // Arrows & Movement
    IconInfo {
        name: "arrow-up",
        display_name: "Arrow Up",
        category: "navigation",
        tags: &["direction", "up", "move"],
        svg_path: "M12 19V5m-7 7 7-7 7 7"
    },
    IconInfo {
        name: "arrow-down",
        display_name: "Arrow Down",
        category: "navigation",
        tags: &["direction", "down", "move"],
        svg_path: "M12 5v14m7-7-7 7-7-7"
    },
    IconInfo {
        name: "refresh-cw",
        display_name: "Refresh Clockwise",
        category: "actions",
        tags: &["reload", "sync", "update"],
        svg_path: "M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8m0 0v-3m0 3h-3m-9 9a9 9 0 0 1-9-9 9.75 9.75 0 0 1 2.74-6.74L6 4m0 0v3m0-3h3"
    },
    IconInfo {
        name: "refresh-ccw",
        display_name: "Refresh Counter-clockwise",
        category: "actions",
        tags: &["reload", "sync", "update"],
        svg_path: "M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16m0 0v3m0-3h3m9-9a9 9 0 0 1 9 9 9.75 9.75 0 0 1-2.74 6.74L18 20m0 0v-3m0 3h-3"
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
