use convert_case::{Case, Casing};
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct SvgIcon {
    name: String,
    paths: Vec<String>,
    other_elements: Vec<String>,
    categories: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct IconMetadata {
    #[serde(default)]
    categories: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    contributors: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Generating Lucide Rust icons...");

    let icons_dir = Path::new("icons");
    if !icons_dir.exists() {
        return Err("Icons directory not found. Please run download script first.".into());
    }

    let svg_files: Vec<PathBuf> = fs::read_dir(icons_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "svg" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    println!("📊 Found {} SVG files", svg_files.len());

    let mut all_icons = Vec::new();
    let mut all_categories = HashSet::new();

    for svg_file in svg_files {
        let filename = svg_file.file_stem().unwrap().to_str().unwrap();
        let svg_content = fs::read_to_string(&svg_file)?;

        // Try to read corresponding JSON metadata
        let json_path = svg_file.with_extension("json");
        let categories = if json_path.exists() {
            match fs::read_to_string(&json_path) {
                Ok(json_content) => match serde_json::from_str::<IconMetadata>(&json_content) {
                    Ok(metadata) => {
                        for category in &metadata.categories {
                            all_categories.insert(category.clone());
                        }
                        metadata.categories
                    }
                    Err(e) => {
                        eprintln!(
                            "⚠️  Warning: Failed to parse metadata for {}: {}",
                            filename, e
                        );
                        vec![]
                    }
                },
                Err(_) => vec![],
            }
        } else {
            vec![]
        };

        match parse_svg(&svg_content, filename, categories) {
            Ok(icon) => {
                all_icons.push(icon);
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to parse {}: {}", filename, e);
            }
        }
    }

    println!("✅ Successfully parsed {} icons", all_icons.len());

    // Generate components for each framework
    generate_dioxus_components(&all_icons)?;
    generate_leptos_components(&all_icons)?;
    generate_yew_components(&all_icons)?;
    generate_sycamore_components(&all_icons)?;

    // Update mod.rs files with category features
    update_mod_files_with_features(&all_icons)?;

    // Generate/update Cargo.toml features
    println!("📋 Updating Cargo.toml with category features...");
    update_cargo_features(&all_categories)?;

    // Generate ICONS.md documentation
    println!("📚 Generating ICONS.md documentation...");
    generate_icons_documentation(&all_icons, &all_categories)?;

    println!("🎉 Generation complete!");
    println!(
        "📊 Found {} categories: {:?}",
        all_categories.len(),
        all_categories.iter().collect::<Vec<_>>()
    );
    Ok(())
}

fn parse_svg(
    content: &str,
    filename: &str,
    categories: Vec<String>,
) -> Result<SvgIcon, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(content);
    let mut paths = Vec::new();
    let mut other_elements = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) | Event::Empty(ref e) => {
                match e.name().as_ref() {
                    b"path" => {
                        let d = e
                            .attributes()
                            .find(|attr| attr.as_ref().map(|a| a.key.as_ref()) == Ok(b"d"))
                            .and_then(|attr| attr.ok())
                            .and_then(|attr| String::from_utf8(attr.value.to_vec()).ok())
                            .unwrap_or_default();
                        paths.push(d);
                    }
                    b"svg" => {} // Skip svg root element
                    _ => {
                        // Handle other elements like circle, rect, polyline, etc.
                        let tag_name = String::from_utf8(e.name().as_ref().to_vec())?;
                        let mut attributes = Vec::new();

                        for attr in e.attributes() {
                            let attr = attr?;
                            let key = String::from_utf8(attr.key.as_ref().to_vec())?;
                            let value = String::from_utf8(attr.value.to_vec())?;
                            attributes.push(format!("{}=\"{}\"", key, value));
                        }

                        let element = if attributes.is_empty() {
                            format!("<{} />", tag_name)
                        } else {
                            format!("<{} {} />", tag_name, attributes.join(" "))
                        };
                        other_elements.push(element);
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(SvgIcon {
        name: filename.to_string(),
        paths,
        other_elements,
        categories,
    })
}

fn to_pascal_case(name: &str) -> String {
    // Component names are clean PascalCase - no prefixes needed
    name.to_case(Case::Pascal)
}

fn to_module_name(name: &str) -> String {
    // All module names get _icon suffix to avoid any keyword conflicts
    format!("{}_icon", name.to_case(Case::Snake))
}

fn generate_dioxus_components(icons: &[SvgIcon]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Generating Dioxus components...");

    for icon in icons {
        let component_name = to_pascal_case(&icon.name);
        let file_name = to_module_name(&icon.name);

        let mut content = String::new();
        content.push_str(&format!(
            r#"use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct {}Props {{
    #[props(default = 24)]
    pub size: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    #[props(default = "none".to_owned())]
    pub fill: String,
    #[props(default = 2)]
    pub stroke_width: usize,
    #[props(default = false)]
    pub absolute_stroke_width: bool,
    pub class: Option<String>,
    pub style: Option<String>,
}}

#[component]
pub fn {}(props: {}Props) -> Element {{
    let stroke_width = if props.absolute_stroke_width {{
        props.stroke_width * 24 / props.size
    }} else {{
        props.stroke_width
    }};
    
    rsx! {{
        svg {{
            "xmlns": "http://www.w3.org/2000/svg",
            "class": if let Some(class) = props.class {{ "{{class}}" }},
            "style": if let Some(style) = props.style {{ "{{style}}" }},
            "width": "{{props.size}}",
            "height": "{{props.size}}",
            "viewBox": "0 0 24 24",
            "fill": "{{props.fill}}",
            "stroke": "{{props.color}}",
            "stroke-width": "{{stroke_width}}",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
"#,
            component_name, component_name, component_name
        ));

        // Add path elements
        for path_d in &icon.paths {
            content.push_str(&format!("            path {{ \"d\": \"{}\" }}\n", path_d));
        }

        // Add other elements (simplified - you might need more sophisticated parsing)
        for element in &icon.other_elements {
            // Convert HTML-style to Dioxus RSX (this is a simple conversion)
            let rsx_element = convert_html_to_dioxus_rsx(element);
            content.push_str(&format!("            {}\n", rsx_element));
        }

        content.push_str(
            r#"        }
    }
}
"#,
        );

        let file_path = format!("src/dioxus/{}.rs", file_name);
        fs::write(&file_path, content)?;
    }

    Ok(())
}

fn generate_leptos_components(icons: &[SvgIcon]) -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Generating Leptos components...");

    for icon in icons {
        let component_name = to_pascal_case(&icon.name);
        let file_name = to_module_name(&icon.name);

        let mut content = String::new();
        content.push_str(&format!(
            r#"use leptos::{{prelude::*, svg::Svg}};

#[component]
pub fn {}(
    #[prop(default = 24.into(), into)] size: Signal<usize>,
    #[prop(default = "currentColor".into(), into)] color: Signal<String>,
    #[prop(default = "none".into(), into)] fill: Signal<String>,
    #[prop(default = 2.into(), into)] stroke_width: Signal<usize>,
    #[prop(default = false.into(), into)] absolute_stroke_width: Signal<bool>,
    #[prop(optional)] node_ref: NodeRef<Svg>,
) -> impl IntoView {{
    let stroke_width = Signal::derive(move || {{
        if absolute_stroke_width.get() {{
            stroke_width.get() * 24 / size.get()
        }} else {{
            stroke_width.get()
        }}
    }});
    
    view! {{
        <svg
            node_ref=node_ref
            class:lucide=true
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || size.get()
            viewBox="0 0 24 24"
            fill=move || fill.get()
            stroke=move || color.get()
            stroke-width=move || stroke_width.get()
            stroke-linecap="round"
            stroke-linejoin="round"
        >
"#,
            component_name
        ));

        // Add path elements
        for path_d in &icon.paths {
            content.push_str(&format!("            <path d=\"{}\" />\n", path_d));
        }

        // Add other elements
        for element in &icon.other_elements {
            content.push_str(&format!("            {}\n", element));
        }

        content.push_str(
            r#"        </svg>
    }
}
"#,
        );

        let file_path = format!("src/leptos/{}.rs", file_name);
        fs::write(&file_path, content)?;
    }

    Ok(())
}

fn generate_yew_components(icons: &[SvgIcon]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🕷️  Generating Yew components...");

    for icon in icons {
        let component_name = to_pascal_case(&icon.name);
        let file_name = to_module_name(&icon.name);

        let mut content = String::new();
        content.push_str(&format!(
            r#"use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct {}Props {{
    #[prop_or(24)]
    pub size: usize,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("none"))]
    pub fill: AttrValue,
    #[prop_or(2)]
    pub stroke_width: usize,
    #[prop_or(false)]
    pub absolute_stroke_width: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: std::option::Option<AttrValue>,
    #[prop_or_default]
    pub node_ref: NodeRef,
}}

#[function_component]
pub fn {}(props: &{}Props) -> Html {{
    let stroke_width = if props.absolute_stroke_width {{
        props.stroke_width * 24 / props.size
    }} else {{
        props.stroke_width
    }};
    
    html! {{
        <svg
            ref={{props.node_ref.clone()}}
            class={{classes!("lucide", props.class.clone())}}
            style={{props.style.clone()}}
            xmlns="http://www.w3.org/2000/svg"
            width={{props.size.to_string()}}
            height={{props.size.to_string()}}
            viewBox="0 0 24 24"
            fill={{&props.fill}}
            stroke={{&props.color}}
            stroke-width={{stroke_width.to_string()}}
            stroke-linecap="round"
            stroke-linejoin="round"
        >
"#,
            component_name, component_name, component_name
        ));

        // Add path elements
        for path_d in &icon.paths {
            content.push_str(&format!("            <path d=\"{}\" />\n", path_d));
        }

        // Add other elements
        for element in &icon.other_elements {
            content.push_str(&format!("            {}\n", element));
        }

        content.push_str(
            r#"        </svg>
    }
}
"#,
        );

        let file_path = format!("src/yew/{}.rs", file_name);
        fs::write(&file_path, content)?;
    }

    Ok(())
}

fn generate_sycamore_components(icons: &[SvgIcon]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌸 Generating Sycamore components...");

    for icon in icons {
        let component_name = to_pascal_case(&icon.name);
        let file_name = to_module_name(&icon.name);

        let mut content = String::new();
        content.push_str(&format!(
            r#"use sycamore::prelude::*;

#[derive(Props)]
pub struct {}Props {{
    #[prop(default = 24)]
    pub size: usize,
    #[prop(default = String::from("currentColor"))]
    pub color: String,
    #[prop(default = String::from("none"))]
    pub fill: String,
    #[prop(default = 2)]
    pub stroke_width: usize,
    #[prop(default = false)]
    pub absolute_stroke_width: bool,
    #[prop(default)]
    pub class: Option<String>,
}}

#[component]
pub fn {}(props: {}Props) -> View {{
    let stroke_width = if props.absolute_stroke_width {{
        props.stroke_width * 24 / props.size
    }} else {{
        props.stroke_width
    }};
    let class = props.class.unwrap_or_default();
    
    view! {{
        svg(
            xmlns="http://www.w3.org/2000/svg",
            class=format!("lucide {{}}", class),
            width=props.size.to_string(),
            height=props.size.to_string(),
            viewBox="0 0 24 24",
            fill=props.fill,
            stroke=props.color,
            "stroke-width"=stroke_width.to_string(),
            "stroke-linecap"="round",
            "stroke-linejoin"="round",
        ) {{
"#,
            component_name, component_name, component_name
        ));

        // Add path elements
        for path_d in &icon.paths {
            content.push_str(&format!("            path(d=\"{}\")\n", path_d));
        }

        // Add other elements (convert to Sycamore syntax)
        for element in &icon.other_elements {
            let sycamore_element = convert_html_to_sycamore_syntax(element);
            content.push_str(&format!("            {}\n", sycamore_element));
        }

        content.push_str(
            r#"        }
    }
}
"#,
        );

        let file_path = format!("src/sycamore/{}.rs", file_name);
        fs::write(&file_path, content)?;
    }

    Ok(())
}

fn update_mod_files(icons: &[SvgIcon]) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Updating mod.rs files...");

    let frameworks = ["dioxus", "leptos", "yew", "sycamore"];

    for framework in &frameworks {
        let mod_path = format!("src/{}/mod.rs", framework);
        let mut mod_content = String::new();

        for icon in icons {
            let file_name = to_module_name(&icon.name);
            mod_content.push_str(&format!("pub mod {};\n", file_name));
        }

        fs::write(&mod_path, mod_content)?
    }

    Ok(())
}

fn convert_html_to_dioxus_rsx(html: &str) -> String {
    // Convert HTML to Dioxus RSX syntax
    if html.contains("polyline") {
        let re = Regex::new(r#"points="([^"]+)""#).unwrap();
        if let Some(captures) = re.captures(html) {
            let points = &captures[1];
            return format!("polyline {{ points: \"{}\" }}", points);
        }
    }

    if html.contains("polygon") {
        let re = Regex::new(r#"points="([^"]+)""#).unwrap();
        if let Some(captures) = re.captures(html) {
            let points = &captures[1];
            return format!("polygon {{ points: \"{}\" }}", points);
        }
    }

    if html.contains("circle") {
        let mut attrs = Vec::new();
        if let Some(cx) = extract_attr(html, "cx") {
            attrs.push(format!("cx: \"{}\"", cx));
        }
        if let Some(cy) = extract_attr(html, "cy") {
            attrs.push(format!("cy: \"{}\"", cy));
        }
        if let Some(r) = extract_attr(html, "r") {
            attrs.push(format!("r: \"{}\"", r));
        }
        return format!("circle {{ {} }}", attrs.join(", "));
    }

    if html.contains("ellipse") {
        let mut attrs = Vec::new();
        if let Some(cx) = extract_attr(html, "cx") {
            attrs.push(format!("cx: \"{}\"", cx));
        }
        if let Some(cy) = extract_attr(html, "cy") {
            attrs.push(format!("cy: \"{}\"", cy));
        }
        if let Some(rx) = extract_attr(html, "rx") {
            attrs.push(format!("rx: \"{}\"", rx));
        }
        if let Some(ry) = extract_attr(html, "ry") {
            attrs.push(format!("ry: \"{}\"", ry));
        }
        return format!("ellipse {{ {} }}", attrs.join(", "));
    }

    if html.contains("rect") {
        let mut attrs = Vec::new();
        if let Some(x) = extract_attr(html, "x") {
            attrs.push(format!("x: \"{}\"", x));
        }
        if let Some(y) = extract_attr(html, "y") {
            attrs.push(format!("y: \"{}\"", y));
        }
        if let Some(width) = extract_attr(html, "width") {
            attrs.push(format!("width: \"{}\"", width));
        }
        if let Some(height) = extract_attr(html, "height") {
            attrs.push(format!("height: \"{}\"", height));
        }
        return format!("rect {{ {} }}", attrs.join(", "));
    }

    if html.contains("line") {
        let mut attrs = Vec::new();
        if let Some(x1) = extract_attr(html, "x1") {
            attrs.push(format!("x1: \"{}\"", x1));
        }
        if let Some(y1) = extract_attr(html, "y1") {
            attrs.push(format!("y1: \"{}\"", y1));
        }
        if let Some(x2) = extract_attr(html, "x2") {
            attrs.push(format!("x2: \"{}\"", x2));
        }
        if let Some(y2) = extract_attr(html, "y2") {
            attrs.push(format!("y2: \"{}\"", y2));
        }
        return format!("line {{ {} }}", attrs.join(", "));
    }

    // Default fallback
    html.to_string()
}

fn convert_html_to_sycamore_syntax(html: &str) -> String {
    // Convert HTML to Sycamore view! syntax
    // This is a simplified conversion
    if html.contains("polyline") {
        let re = Regex::new(r#"points="([^"]+)""#).unwrap();
        if let Some(captures) = re.captures(html) {
            let points = &captures[1];
            return format!("polyline(points=\"{}\")", points);
        }
    }

    if html.contains("polygon") {
        let re = Regex::new(r#"points="([^"]+)""#).unwrap();
        if let Some(captures) = re.captures(html) {
            let points = &captures[1];
            return format!("polygon(points=\"{}\")", points);
        }
    }

    if html.contains("circle") {
        // Extract circle attributes
        let mut attrs = Vec::new();
        if let Some(cx) = extract_attr(html, "cx") {
            attrs.push(format!("cx=\"{}\"", cx));
        }
        if let Some(cy) = extract_attr(html, "cy") {
            attrs.push(format!("cy=\"{}\"", cy));
        }
        if let Some(r) = extract_attr(html, "r") {
            attrs.push(format!("r=\"{}\"", r));
        }
        return format!("circle({})", attrs.join(", "));
    }

    if html.contains("ellipse") {
        // Extract ellipse attributes
        let mut attrs = Vec::new();
        if let Some(cx) = extract_attr(html, "cx") {
            attrs.push(format!("cx=\"{}\"", cx));
        }
        if let Some(cy) = extract_attr(html, "cy") {
            attrs.push(format!("cy=\"{}\"", cy));
        }
        if let Some(rx) = extract_attr(html, "rx") {
            attrs.push(format!("rx=\"{}\"", rx));
        }
        if let Some(ry) = extract_attr(html, "ry") {
            attrs.push(format!("ry=\"{}\"", ry));
        }
        return format!("ellipse({})", attrs.join(", "));
    }

    if html.contains("rect") {
        // Extract rect attributes
        let mut attrs = Vec::new();
        if let Some(x) = extract_attr(html, "x") {
            attrs.push(format!("x=\"{}\"", x));
        }
        if let Some(y) = extract_attr(html, "y") {
            attrs.push(format!("y=\"{}\"", y));
        }
        if let Some(width) = extract_attr(html, "width") {
            attrs.push(format!("width=\"{}\"", width));
        }
        if let Some(height) = extract_attr(html, "height") {
            attrs.push(format!("height=\"{}\"", height));
        }
        return format!("rect({})", attrs.join(", "));
    }

    if html.contains("line") {
        let mut attrs = Vec::new();
        if let Some(x1) = extract_attr(html, "x1") {
            attrs.push(format!("x1=\"{}\"", x1));
        }
        if let Some(y1) = extract_attr(html, "y1") {
            attrs.push(format!("y1=\"{}\"", y1));
        }
        if let Some(x2) = extract_attr(html, "x2") {
            attrs.push(format!("x2=\"{}\"", x2));
        }
        if let Some(y2) = extract_attr(html, "y2") {
            attrs.push(format!("y2=\"{}\"", y2));
        }
        return format!("line({})", attrs.join(", "));
    }

    // Default fallback
    html.to_string()
}

fn extract_attr(html: &str, attr_name: &str) -> Option<String> {
    let re = Regex::new(&format!(r#"{}="([^"]+)""#, attr_name)).ok()?;
    re.captures(html)?.get(1).map(|m| m.as_str().to_string())
}

fn update_cargo_features(categories: &HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_path = "Cargo.toml";
    let content = fs::read_to_string(cargo_path)?;

    // Find the [features] section and replace it
    let mut lines: Vec<&str> = content.lines().collect();
    let mut features_start = None;
    let mut features_end = None;

    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "[features]" {
            features_start = Some(i);
        } else if features_start.is_some() && line.starts_with('[') && *line != "[features]" {
            features_end = Some(i);
            break;
        }
    }

    if let Some(start) = features_start {
        let end = features_end.unwrap_or(lines.len());

        // Build new features section as string
        let mut features_content = String::new();
        features_content.push_str("[features]\n");
        features_content.push_str("default = []\n");
        features_content.push_str("\n");
        features_content.push_str("# Framework features\n");
        features_content.push_str("dioxus = [\"dep:dioxus\"]\n");
        features_content.push_str("leptos = [\"dep:leptos\"]\n");
        features_content.push_str("yew = [\"dep:yew\"]\n");
        features_content.push_str("sycamore = [\"dep:sycamore\"]\n");
        features_content.push_str("codegen = [\"dep:quick-xml\", \"dep:regex\", \"dep:convert_case\", \"dep:serde\", \"dep:serde_json\"]\n");
        features_content.push_str("\n");
        features_content.push_str("# Icon category features for bundle size optimization\n");

        // Add category features
        let mut sorted_categories: Vec<&String> = categories.iter().collect();
        sorted_categories.sort();

        for category in &sorted_categories {
            features_content.push_str(&format!("{} = []\n", category));
        }

        features_content.push_str("\n");
        features_content.push_str("# Meta features\n");
        features_content.push_str("all-icons = [\n");

        // Add all categories to all-icons feature
        for (i, category) in sorted_categories.iter().enumerate() {
            if i == sorted_categories.len() - 1 {
                features_content.push_str(&format!("    \"{}\",\n", category));
            } else {
                features_content.push_str(&format!("    \"{}\",\n", category));
            }
        }

        features_content.push_str("]\n");
        features_content.push_str("\n");
        features_content.push_str("# Common combinations for convenience\n");
        features_content
            .push_str("essentials = [\"arrows\", \"navigation\", \"files\", \"communication\"]\n");
        features_content.push_str("web-app = [\"arrows\", \"navigation\", \"files\", \"communication\", \"layout\", \"notifications\"]\n");
        features_content.push_str("mobile-app = [\"arrows\", \"navigation\", \"communication\", \"multimedia\", \"account\"]\n\n");

        // Replace the features section in content
        let before_features = lines[..start].join("\n");
        let after_features = lines[end..].join("\n");
        let new_content = format!(
            "{}\n{}\n{}",
            before_features,
            features_content.trim_end(),
            after_features
        );

        fs::write(cargo_path, new_content)?;

        println!(
            "✅ Updated Cargo.toml with {} category features",
            categories.len()
        );
    }

    Ok(())
}

fn update_mod_files_with_features(icons: &[SvgIcon]) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Updating mod.rs files with conditional compilation...");

    let frameworks = ["dioxus", "leptos", "yew", "sycamore"];

    for framework in &frameworks {
        let mod_path = format!("src/{}/mod.rs", framework);
        let mut mod_content = String::new();

        // Build a map of icon name -> all categories it belongs to
        let mut icon_to_categories: HashMap<String, Vec<String>> = HashMap::new();
        let mut all_categories: HashSet<String> = HashSet::new();
        let mut uncategorized_icons = Vec::new();

        for icon in icons {
            let icon_name = icon.name.clone();
            if icon.categories.is_empty() {
                uncategorized_icons.push(icon);
            } else {
                // Store all categories for this icon
                icon_to_categories.insert(icon_name.clone(), icon.categories.clone());

                // Collect all unique categories
                for category in &icon.categories {
                    all_categories.insert(category.clone());
                }
            }
        }

        // Get all icons (no duplicates)
        let mut all_icon_names: HashSet<String> = icon_to_categories.keys().cloned().collect();
        let mut sorted_icon_names: Vec<String> = all_icon_names.into_iter().collect();
        sorted_icon_names.sort();

        // Generate each icon with cfg attributes for all its categories
        for icon_name in sorted_icon_names {
            let file_name = to_module_name(&icon_name);
            if let Some(categories) = icon_to_categories.get(&icon_name) {
                // Build cfg attribute with all categories this icon belongs to
                let mut cfg_features = Vec::new();
                for category in categories {
                    cfg_features.push(format!("feature = \"{}\"", category));
                }
                cfg_features.push("feature = \"all-icons\"".to_string());

                let cfg_attr = format!("#[cfg(any({}))]\n", cfg_features.join(", "));
                mod_content.push_str(&cfg_attr);
                mod_content.push_str(&format!("pub mod {};\n", file_name));
            }
        }

        // Add uncategorized icons (always available)
        if !uncategorized_icons.is_empty() {
            mod_content.push_str("\n// Uncategorized icons (always available)\n");
            for icon in uncategorized_icons {
                let file_name = to_module_name(&icon.name);
                mod_content.push_str(&format!("pub mod {};\n", file_name));
            }
        }

        fs::write(&mod_path, mod_content)?
    }

    Ok(())
}

fn generate_icons_documentation(
    icons: &[SvgIcon],
    categories: &HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();

    // Header
    content.push_str("# 🎨 Lucide Icons Reference\n\n");
    content.push_str(&format!("This document lists all {} available icons in the Lucide Rust library, organized by category.\n\n", icons.len()));

    // Table of contents
    content.push_str("## 📚 Table of Contents\n\n");

    let mut sorted_categories: Vec<&String> = categories.iter().collect();
    sorted_categories.sort();

    for category in &sorted_categories {
        let category_title = category.replace('-', " ").to_case(Case::Title);
        let anchor = category.to_lowercase().replace(' ', "-");
        content.push_str(&format!(
            "- [{}](#{})\
",
            category_title, anchor
        ));
    }

    // Usage section
    content.push_str("\n## 🚀 Usage\n\n");
    content.push_str("### Basic Usage\n\n");
    content.push_str("```rust\n");
    content.push_str("use lucide::*;\n\n");
    content.push_str("// In your component\n");
    content.push_str("<Home class=\"w-6 h-6\" />\n");
    content.push_str("```\n\n");

    content.push_str("### With Specific Framework\n\n");
    content.push_str("Add to your `Cargo.toml`:\n\n");
    content.push_str("```toml\n");
    content.push_str("# For specific frameworks\n");
    content.push_str("lucide = { version = \"0.1.0\", features = [\"leptos\"] }\n");
    content.push_str("lucide = { version = \"0.1.0\", features = [\"yew\"] }\n");
    content.push_str("lucide = { version = \"0.1.0\", features = [\"dioxus\"] }\n");
    content.push_str("lucide = { version = \"0.1.0\", features = [\"sycamore\"] }\n\n");
    content.push_str("# With specific icon categories (for smaller bundle size)\n");
    content.push_str(
        "lucide = { version = \"0.1.0\", features = [\"leptos\", \"navigation\", \"files\"] }\n\n",
    );
    content.push_str("# Or include all icons with:\n");
    content.push_str("lucide = { version = \"0.1.0\", features = [\"leptos\", \"all-icons\"] }\n");
    content.push_str("```\n\n");
    // Icons by category
    content.push_str("## 🗂️ Icons by Category\n\n");

    // Group icons by category
    let mut category_icons: HashMap<String, Vec<&SvgIcon>> = HashMap::new();
    let mut uncategorized_icons = Vec::new();

    for icon in icons {
        if icon.categories.is_empty() {
            uncategorized_icons.push(icon);
        } else {
            for category in &icon.categories {
                category_icons
                    .entry(category.clone())
                    .or_insert_with(Vec::new)
                    .push(icon);
            }
        }
    }

    // Sort and display each category
    for category in &sorted_categories {
        if let Some(icons_in_category) = category_icons.get(*category) {
            let category_title = category.replace('-', " ").to_case(Case::Title);
            content.push_str(&format!("### {}\n\n", category_title));
            content.push_str(&format!(
                "`{}` and `all-icons` features - {} icons\n\n",
                category,
                icons_in_category.len()
            ));

            // Create a table of icons in this category
            content.push_str("| Icon | Name | Component |\n");
            content.push_str("|------|------|-----------|\n");

            let mut sorted_icons = icons_in_category.clone();
            sorted_icons.sort_by(|a, b| a.name.cmp(&b.name));

            for icon in &sorted_icons {
                let component_name = to_pascal_case(&icon.name);
                let icon_preview =
                    format!("[{}](https://lucide.dev/icons/{})", icon.name, icon.name);
                content.push_str(&format!(
                    "| {} | `{}` | `<{} />` |\n",
                    icon_preview, icon.name, component_name
                ));
            }

            content.push_str("\n");

            // Usage example for this category
            if let Some(first_icon) = sorted_icons.first() {
                let component_name = to_pascal_case(&first_icon.name);
                content.push_str(&format!("**Usage example:**\n\n"));
                content.push_str("```rust\n");
                content.push_str(&format!("// Add to Cargo.toml: features = [\"leptos\", \"{}\"] or [\"leptos\", \"all-icons\"]\n", category));
                content.push_str(&format!(
                    "<{} class=\"w-6 h-6 text-gray-600\" />\n",
                    component_name
                ));
                content.push_str("```\n\n");
            }
        }
    }

    // Add uncategorized icons if any
    if !uncategorized_icons.is_empty() {
        content.push_str("### Uncategorized\n\n");
        content.push_str(&format!("{} icons\n\n", uncategorized_icons.len()));

        content.push_str("| Icon | Name | Component |\n");
        content.push_str("|------|------|-----------|\n");

        uncategorized_icons.sort_by(|a, b| a.name.cmp(&b.name));

        for icon in uncategorized_icons {
            let component_name = to_pascal_case(&icon.name);
            let icon_preview = format!("[{}](https://lucide.dev/icons/{})", icon.name, icon.name);
            content.push_str(&format!(
                "| {} | `{}` | `<{} />` |\n",
                icon_preview, icon.name, component_name
            ));
        }

        content.push_str("\n");
    }

    // Footer
    content.push_str("---\n\n");
    content.push_str("*This file is automatically generated by the `generate-icons` script.*\n");

    // Write to repository root directory
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "".to_string());
    let root_path = Path::new(&manifest_dir).join("../../ICONS.md");
    fs::write(&root_path, content)?;

    println!(
        "✅ Generated ICONS.md at {} with {} icons across {} categories",
        root_path.display(),
        icons.len(),
        categories.len()
    );

    Ok(())
}
