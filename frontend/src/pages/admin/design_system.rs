use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};

#[derive(Debug, Clone, PartialEq)]
pub struct AdminColorScheme {
    pub name: String,
    pub primary: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub border: String,
    
    // Header styling
    pub header_gradient: String,
    pub header_text_color: String,
    pub header_border_color: String,
    pub header_shadow: String,
    pub header_text_shadow: String,
    pub header_logo_gradient: String,
    
    // Sidebar styling
    pub sidebar_bg: String,
    pub sidebar_border_color: String,
    pub sidebar_shadow: String,
    pub sidebar_section_title_color: String,
    pub sidebar_section_border_color: String,
    
    // Navigation links
    pub nav_link_text_color: String,
    pub nav_link_hover_bg: String,
    pub nav_link_hover_text: String,
    pub nav_link_active_bg: String,
    pub nav_link_active_shadow: String,
    pub nav_link_active_indicator: String,
    pub nav_link_public_text: String,
    pub nav_link_public_hover_bg: String,
    pub nav_link_public_hover_text: String,
    
    // General layout
    pub card_bg: String,
    pub shadow_color: String,
    pub accent_color: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PublicColorScheme {
    pub name: String,
    // Primary text hierarchy
    pub text_primary: String,
    pub text_secondary: String,
    pub text_meta: String,
    pub text_light: String,
    pub text_muted: String,
    
    // Link colors with states
    pub link_primary: String,
    pub link_hover: String,
    pub link_visited: String,
    pub link_active: String,
    
    // Heading hierarchy
    pub heading_h1: String,
    pub heading_h2: String,
    pub heading_h3: String,
    pub heading_h4: String,
    pub heading_h5: String,
    pub heading_h6: String,
    
    // Context-specific text
    pub header_text: String,
    pub header_text_hover: String,
    pub footer_text: String,
    pub footer_text_muted: String,
    
    // Semantic colors
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
    
    // Layout colors
    pub border_light: String,
    pub background_light: String,
    pub header_bg: String,
    pub footer_bg: String,
    pub hero_bg: String,
    pub card_shadow: String,
}

impl Default for AdminColorScheme {
    fn default() -> Self {
        Self {
            name: "Admin Dark Theme".to_string(),
            // Dark theme color palette
            primary: "#3b82f6".to_string(),        // Vibrant blue
            secondary: "#6b7280".to_string(),      // Gray 500
            success: "#10b981".to_string(),        // Emerald green
            warning: "#f59e0b".to_string(),        // Amber
            danger: "#ef4444".to_string(),         // Red
            info: "#06b6d4".to_string(),           // Cyan
            
            // Dark layout colors
            background: "#1f2937".to_string(),     // Gray 800 - dark background
            surface: "#374151".to_string(),        // Gray 700 - surface color
            text_primary: "#f9fafb".to_string(),   // Gray 50 - primary text
            text_secondary: "#d1d5db".to_string(), // Gray 300 - secondary text
            border: "#4b5563".to_string(),         // Gray 600 - borders
            
            // Dark theme header styling
            header_gradient: "linear-gradient(135deg, #1e293b 0%, #334155 50%, #475569 100%)".to_string(),
            header_text_color: "#f9fafb".to_string(),
            header_border_color: "rgba(255, 255, 255, 0.2)".to_string(),
            header_shadow: "0 4px 20px rgba(0, 0, 0, 0.5)".to_string(),
            header_text_shadow: "0 2px 4px rgba(0, 0, 0, 0.5)".to_string(),
            header_logo_gradient: "linear-gradient(135deg, #3b82f6, #1e40af)".to_string(),
            
            // Dark theme sidebar
            sidebar_bg: "linear-gradient(180deg, #1e293b 0%, #334155 50%, #475569 100%)".to_string(),
            sidebar_border_color: "rgba(255, 255, 255, 0.2)".to_string(),
            sidebar_shadow: "4px 0 20px rgba(0, 0, 0, 0.5)".to_string(),
            sidebar_section_title_color: "rgba(255, 255, 255, 0.9)".to_string(),
            sidebar_section_border_color: "rgba(255, 255, 255, 0.2)".to_string(),
            
            // Dark theme navigation
            nav_link_text_color: "rgba(255, 255, 255, 0.9)".to_string(),
            nav_link_hover_bg: "rgba(255, 255, 255, 0.15)".to_string(),
            nav_link_hover_text: "#ffffff".to_string(),
            nav_link_active_bg: "linear-gradient(135deg, #3b82f6, #1e40af)".to_string(),
            nav_link_active_shadow: "0 4px 15px rgba(59, 130, 246, 0.4)".to_string(),
            nav_link_active_indicator: "#ffffff".to_string(),
            nav_link_public_text: "rgba(255, 255, 255, 0.7)".to_string(),
            nav_link_public_hover_bg: "rgba(255, 255, 255, 0.08)".to_string(),
            nav_link_public_hover_text: "rgba(255, 255, 255, 0.9)".to_string(),
            
            // Dark theme layout styling
            card_bg: "#374151".to_string(),
            shadow_color: "0 10px 15px -3px rgba(0, 0, 0, 0.3), 0 4px 6px -2px rgba(0, 0, 0, 0.2)".to_string(),
            accent_color: "#3b82f6".to_string(),
        }
    }
}

impl Default for PublicColorScheme {
    fn default() -> Self {
        Self {
            name: "Modern Editorial".to_string(),
            // Enhanced typography hierarchy for optimal readability
            text_primary: "#0f172a".to_string(),    // Slate 900 - strong readability
            text_secondary: "#334155".to_string(),  // Slate 700 - body text
            text_meta: "#64748b".to_string(),       // Slate 500 - metadata
            text_light: "#94a3b8".to_string(),      // Slate 400 - captions
            text_muted: "#cbd5e1".to_string(),      // Slate 300 - subtle text
            
            // Sophisticated link system
            link_primary: "#2563eb".to_string(),    // Blue 600 - accessible primary
            link_hover: "#1d4ed8".to_string(),      // Blue 700 - hover state
            link_visited: "#7c3aed".to_string(),    // Violet 600 - visited
            link_active: "#1e40af".to_string(),     // Blue 800 - active state
            
            // Professional heading hierarchy
            heading_h1: "#0f172a".to_string(),      // Slate 900 - maximum impact
            heading_h2: "#1e293b".to_string(),      // Slate 800 - section headers
            heading_h3: "#334155".to_string(),      // Slate 700 - subsections
            heading_h4: "#475569".to_string(),      // Slate 600 - minor headings
            heading_h5: "#64748b".to_string(),      // Slate 500 - small headings
            heading_h6: "#64748b".to_string(),      // Slate 500 - smallest headings
            
            // Modern header and footer styling
            header_text: "#f8fafc".to_string(),         // Slate 50 - clean contrast
            header_text_hover: "#e2e8f0".to_string(),   // Slate 200 - subtle hover
            footer_text: "#f1f5f9".to_string(),         // Slate 100 - footer clarity
            footer_text_muted: "#cbd5e1".to_string(),   // Slate 300 - footer secondary
            
            // Professional semantic colors
            success: "#10b981".to_string(),         // Emerald 500 - positive actions
            warning: "#f59e0b".to_string(),         // Amber 500 - cautions
            danger: "#ef4444".to_string(),          // Red 500 - errors
            info: "#06b6d4".to_string(),            // Cyan 500 - information
            
            // Refined layout and styling
            border_light: "#e2e8f0".to_string(),    // Slate 200 - subtle borders
            background_light: "#f8fafc".to_string(), // Slate 50 - clean backgrounds
            header_bg: "#000000".to_string(),        // Black header background
            footer_bg: "#000000".to_string(),        // Black footer background
            hero_bg: "linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%)".to_string(), // Elegant gradient
            card_shadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)".to_string(), // Modern shadow
        }
    }
}

#[function_component(DesignSystemPage)]
pub fn design_system_page() -> Html {
    let admin_scheme = use_state(|| AdminColorScheme::default());
    let public_scheme = use_state(|| PublicColorScheme::default());
    let current_tab = use_state(|| "admin".to_string());
    let saved_themes = use_state(|| vec![
        "Light Preset".to_string(),
        "Dark Preset".to_string(),
    ]);
    let selected_preset = use_state(|| "Light Preset".to_string());
    let theme_name_input = use_state(|| String::new());

    let switch_tab = {
        let current_tab = current_tab.clone();
        Callback::from(move |tab: String| {
            current_tab.set(tab);
        })
    };

    let update_admin_color = {
        let admin_scheme = admin_scheme.clone();
        Callback::from(move |(property, value): (String, String)| {
            let mut scheme = (*admin_scheme).clone();
            match property.as_str() {
                "primary" => scheme.primary = value,
                "secondary" => scheme.secondary = value,
                "success" => scheme.success = value,
                "warning" => scheme.warning = value,
                "danger" => scheme.danger = value,
                "info" => scheme.info = value,
                "background" => scheme.background = value,
                "surface" => scheme.surface = value,
                "text_primary" => scheme.text_primary = value,
                "text_secondary" => scheme.text_secondary = value,
                "border" => scheme.border = value,
                
                // Header styling
                "header_gradient" => scheme.header_gradient = value,
                "header_text_color" => scheme.header_text_color = value,
                "header_border_color" => scheme.header_border_color = value,
                "header_shadow" => scheme.header_shadow = value,
                "header_text_shadow" => scheme.header_text_shadow = value,
                "header_logo_gradient" => scheme.header_logo_gradient = value,
                
                // Sidebar styling
                "sidebar_bg" => scheme.sidebar_bg = value,
                "sidebar_border_color" => scheme.sidebar_border_color = value,
                "sidebar_shadow" => scheme.sidebar_shadow = value,
                "sidebar_section_title_color" => scheme.sidebar_section_title_color = value,
                "sidebar_section_border_color" => scheme.sidebar_section_border_color = value,
                
                // Navigation links
                "nav_link_text_color" => scheme.nav_link_text_color = value,
                "nav_link_hover_bg" => scheme.nav_link_hover_bg = value,
                "nav_link_hover_text" => scheme.nav_link_hover_text = value,
                "nav_link_active_bg" => scheme.nav_link_active_bg = value,
                "nav_link_active_shadow" => scheme.nav_link_active_shadow = value,
                "nav_link_active_indicator" => scheme.nav_link_active_indicator = value,
                "nav_link_public_text" => scheme.nav_link_public_text = value,
                "nav_link_public_hover_bg" => scheme.nav_link_public_hover_bg = value,
                "nav_link_public_hover_text" => scheme.nav_link_public_hover_text = value,
                
                // General layout
                "card_bg" => scheme.card_bg = value,
                "shadow_color" => scheme.shadow_color = value,
                "accent_color" => scheme.accent_color = value,
                _ => {}
            }
            admin_scheme.set(scheme);
        })
    };

    let update_public_color = {
        let public_scheme = public_scheme.clone();
        Callback::from(move |(property, value): (String, String)| {
            let mut scheme = (*public_scheme).clone();
            match property.as_str() {
                // Primary text hierarchy
                "text_primary" => scheme.text_primary = value,
                "text_secondary" => scheme.text_secondary = value,
                "text_meta" => scheme.text_meta = value,
                "text_light" => scheme.text_light = value,
                "text_muted" => scheme.text_muted = value,
                
                // Link colors with states
                "link_primary" => scheme.link_primary = value,
                "link_hover" => scheme.link_hover = value,
                "link_visited" => scheme.link_visited = value,
                "link_active" => scheme.link_active = value,
                
                // Heading hierarchy
                "heading_h1" => scheme.heading_h1 = value,
                "heading_h2" => scheme.heading_h2 = value,
                "heading_h3" => scheme.heading_h3 = value,
                "heading_h4" => scheme.heading_h4 = value,
                "heading_h5" => scheme.heading_h5 = value,
                "heading_h6" => scheme.heading_h6 = value,
                
                // Context-specific text
                "header_text" => scheme.header_text = value,
                "header_text_hover" => scheme.header_text_hover = value,
                "footer_text" => scheme.footer_text = value,
                "footer_text_muted" => scheme.footer_text_muted = value,
                
                // Semantic colors
                "success" => scheme.success = value,
                "warning" => scheme.warning = value,
                "danger" => scheme.danger = value,
                "info" => scheme.info = value,
                
                // Layout colors
                "border_light" => scheme.border_light = value,
                "background_light" => scheme.background_light = value,
                "header_bg" => scheme.header_bg = value,
                "footer_bg" => scheme.footer_bg = value,
                "hero_bg" => scheme.hero_bg = value,
                "card_shadow" => scheme.card_shadow = value,
                
                _ => {}
            }
            public_scheme.set(scheme);
        })
    };

    let apply_admin_theme = {
        let admin_scheme = admin_scheme.clone();
        Callback::from(move |_: MouseEvent| {
            apply_admin_css_variables(&*admin_scheme);
        })
    };

    let apply_public_theme = {
        let public_scheme = public_scheme.clone();
        Callback::from(move |_: MouseEvent| {
            apply_public_css_variables(&*public_scheme);
        })
    };

    let reset_admin_defaults = {
        let admin_scheme = admin_scheme.clone();
        Callback::from(move |_: MouseEvent| {
            admin_scheme.set(AdminColorScheme::default());
        })
    };

    let reset_public_defaults = {
        let public_scheme = public_scheme.clone();
        Callback::from(move |_: MouseEvent| {
            public_scheme.set(PublicColorScheme::default());
        })
    };

    // Removed unused preset callbacks - functionality moved to on_preset_change

    // Removed unused preset callbacks - functionality moved to on_preset_change

    let on_preset_change = {
        let selected_preset = selected_preset.clone();
        let admin_scheme = admin_scheme.clone();
        let public_scheme = public_scheme.clone();
        let current_tab = current_tab.clone();
        Callback::from(move |event: web_sys::Event| {
            let input = event.target().unwrap().dyn_into::<HtmlSelectElement>().unwrap();
            let preset_name = input.value();
            selected_preset.set(preset_name.clone());
            
            // Load the preset based on current tab and selection
            let current_tab_val = (*current_tab).clone();
            match (current_tab_val.as_str(), preset_name.as_str()) {
                ("admin", "Dark Preset") => {
                    let scheme = AdminColorScheme {
                        name: "Admin Dark Theme".to_string(),
                        primary: "#3b82f6".to_string(),
                        secondary: "#6b7280".to_string(),
                        success: "#10b981".to_string(),
                        warning: "#f59e0b".to_string(),
                        danger: "#ef4444".to_string(),
                        info: "#06b6d4".to_string(),
                        background: "#1f2937".to_string(),
                        surface: "#374151".to_string(),
                        text_primary: "#f9fafb".to_string(),
                        text_secondary: "#d1d5db".to_string(),
                        border: "#4b5563".to_string(),
                        header_gradient: "linear-gradient(135deg, #1e293b 0%, #334155 50%, #475569 100%)".to_string(),
                        sidebar_bg: "linear-gradient(180deg, #1e293b 0%, #334155 50%, #475569 100%)".to_string(),
                        // Header styling - dark theme defaults
                        header_text_color: "#f9fafb".to_string(),
                        header_border_color: "rgba(255, 255, 255, 0.2)".to_string(),
                        header_shadow: "0 4px 20px rgba(0, 0, 0, 0.5)".to_string(),
                        header_text_shadow: "0 2px 4px rgba(0, 0, 0, 0.5)".to_string(),
                        header_logo_gradient: "linear-gradient(135deg, #3b82f6, #1e40af)".to_string(),
                        
                        // Sidebar styling - dark theme defaults
                        sidebar_border_color: "rgba(255, 255, 255, 0.2)".to_string(),
                        sidebar_shadow: "4px 0 20px rgba(0, 0, 0, 0.5)".to_string(),
                        sidebar_section_title_color: "rgba(255, 255, 255, 0.9)".to_string(),
                        sidebar_section_border_color: "rgba(255, 255, 255, 0.2)".to_string(),
                        
                        // Navigation link styling - dark theme defaults
                        nav_link_text_color: "rgba(255, 255, 255, 0.9)".to_string(),
                        nav_link_hover_bg: "rgba(255, 255, 255, 0.15)".to_string(),
                        nav_link_hover_text: "#ffffff".to_string(),
                        nav_link_active_bg: "linear-gradient(135deg, #3b82f6, #1e40af)".to_string(),
                        nav_link_active_shadow: "0 4px 15px rgba(59, 130, 246, 0.4)".to_string(),
                        nav_link_active_indicator: "#ffffff".to_string(),
                        nav_link_public_text: "rgba(255, 255, 255, 0.7)".to_string(),
                        nav_link_public_hover_bg: "rgba(255, 255, 255, 0.08)".to_string(),
                        nav_link_public_hover_text: "rgba(255, 255, 255, 0.9)".to_string(),
                        card_bg: "#374151".to_string(),
                        shadow_color: "0 4px 6px -1px rgb(0 0 0 / 0.3), 0 2px 4px -2px rgb(0 0 0 / 0.3)".to_string(),
                        accent_color: "#60a5fa".to_string(),
                    };
                    admin_scheme.set(scheme.clone());
                    apply_admin_css_variables(&scheme);
                },
                ("admin", "Light Preset") => {
                    let scheme = AdminColorScheme {
                        name: "Admin Light Theme".to_string(),
                        primary: "#2563eb".to_string(),
                        secondary: "#64748b".to_string(),
                        success: "#059669".to_string(),
                        warning: "#d97706".to_string(),
                        danger: "#dc2626".to_string(),
                        info: "#0891b2".to_string(),
                        // Light theme colors
                        background: "#f8fafc".to_string(),     // Light gray-blue
                        surface: "#ffffff".to_string(),        // White
                        text_primary: "#1e293b".to_string(),   // Dark text
                        text_secondary: "#64748b".to_string(), // Gray text
                        border: "#e2e8f0".to_string(),         // Light gray border
                        header_gradient: "linear-gradient(135deg, #4c1d95 0%, #312e81 50%, #1e1b4b 100%)".to_string(),
                        sidebar_bg: "linear-gradient(180deg, #4c1d95 0%, #312e81 50%, #1e1b4b 100%)".to_string(),
                        // Header styling - light theme defaults (same as default)
                        header_text_color: "white".to_string(),
                        header_border_color: "rgba(255, 255, 255, 0.1)".to_string(),
                        header_shadow: "0 4px 20px rgba(0, 0, 0, 0.3)".to_string(),
                        header_text_shadow: "0 2px 4px rgba(0, 0, 0, 0.3)".to_string(),
                        header_logo_gradient: "linear-gradient(135deg, #667eea, #764ba2)".to_string(),
                        
                        // Sidebar styling - light theme defaults (same as default)
                        sidebar_border_color: "rgba(255, 255, 255, 0.1)".to_string(),
                        sidebar_shadow: "4px 0 20px rgba(0, 0, 0, 0.3)".to_string(),
                        sidebar_section_title_color: "rgba(255, 255, 255, 0.8)".to_string(),
                        sidebar_section_border_color: "rgba(255, 255, 255, 0.1)".to_string(),
                        
                        // Navigation link styling - light theme defaults (same as default)
                        nav_link_text_color: "rgba(255, 255, 255, 0.8)".to_string(),
                        nav_link_hover_bg: "rgba(255, 255, 255, 0.1)".to_string(),
                        nav_link_hover_text: "white".to_string(),
                        nav_link_active_bg: "linear-gradient(135deg, #667eea, #764ba2)".to_string(),
                        nav_link_active_shadow: "0 4px 15px rgba(102, 126, 234, 0.3)".to_string(),
                        nav_link_active_indicator: "white".to_string(),
                        nav_link_public_text: "rgba(255, 255, 255, 0.6)".to_string(),
                        nav_link_public_hover_bg: "rgba(255, 255, 255, 0.05)".to_string(),
                        nav_link_public_hover_text: "rgba(255, 255, 255, 0.8)".to_string(),
                        card_bg: "#ffffff".to_string(),
                        shadow_color: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
                        accent_color: "#667eea".to_string(),
                    };
                    admin_scheme.set(scheme.clone());
                    apply_admin_css_variables(&scheme);
                },
                ("public", "Dark Preset") => {
                    let scheme = PublicColorScheme {
                        name: "Public Dark Theme".to_string(),
                        // Dark theme text hierarchy
                        text_primary: "#f8fafc".to_string(),
                        text_secondary: "#e2e8f0".to_string(),
                        text_meta: "#cbd5e1".to_string(),
                        text_light: "#94a3b8".to_string(),
                        text_muted: "#64748b".to_string(),
                        
                        // Dark theme link colors
                        link_primary: "#60a5fa".to_string(),
                        link_hover: "#93c5fd".to_string(),
                        link_visited: "#a78bfa".to_string(),
                        link_active: "#3b82f6".to_string(),
                        
                        // Dark theme heading hierarchy
                        heading_h1: "#f8fafc".to_string(),
                        heading_h2: "#f1f5f9".to_string(),
                        heading_h3: "#e2e8f0".to_string(),
                        heading_h4: "#cbd5e1".to_string(),
                        heading_h5: "#94a3b8".to_string(),
                        heading_h6: "#64748b".to_string(),
                        
                        // Dark theme context-specific text
                        header_text: "#f8fafc".to_string(),
                        header_text_hover: "#cbd5e1".to_string(),
                        footer_text: "#e2e8f0".to_string(),
                        footer_text_muted: "#94a3b8".to_string(),
                        
                        // Dark theme semantic colors
                        success: "#10b981".to_string(),
                        warning: "#f59e0b".to_string(),
                        danger: "#ef4444".to_string(),
                        info: "#06b6d4".to_string(),
                        
                        // Dark theme layout colors
                        border_light: "#334155".to_string(),
                        background_light: "#1e293b".to_string(),
                        header_bg: "#1e293b".to_string(),
                        footer_bg: "#1e293b".to_string(),
                        hero_bg: "#0f172a".to_string(),
                        card_shadow: "rgba(0, 0, 0, 0.3)".to_string(),
                    };
                    public_scheme.set(scheme.clone());
                    apply_public_css_variables(&scheme);
                },
                ("public", "Light Preset") => {
                    let scheme = PublicColorScheme::default();
                    public_scheme.set(scheme.clone());
                    apply_public_css_variables(&scheme);
                },
                _ => {}
            }
        })
    };

    let save_theme = {
        let theme_name_input = theme_name_input.clone();
        let saved_themes = saved_themes.clone();
        let _current_tab = current_tab.clone();
        Callback::from(move |_: MouseEvent| {
            let theme_name = (*theme_name_input).clone();
            if !theme_name.is_empty() {
                let mut themes = (*saved_themes).clone();
                if !themes.contains(&theme_name) {
                    themes.push(theme_name);
                    saved_themes.set(themes);
                }
                theme_name_input.set(String::new());
            }
        })
    };

    let on_theme_name_change = {
        let theme_name_input = theme_name_input.clone();
        Callback::from(move |event: web_sys::Event| {
            let input = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            theme_name_input.set(input.value());
        })
    };

    html! {
        <div class="design-system-page">
            <div class="page-header">
                <h1>{"🎨 Design System"}</h1>
                <p>{"Manage themes and styling for admin and public interfaces separately"}</p>
            </div>

            <div class="design-system-tabs">
                <button 
                    class={if *current_tab == "admin" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("admin".to_string())}
                >
                    {"🛠️ Admin Theme"}
                </button>
                <button 
                    class={if *current_tab == "public" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("public".to_string())}
                >
                    {"🌐 Public Theme"}
                </button>
                <button 
                    class={if *current_tab == "typography" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("typography".to_string())}
                >
                    {"📝 Typography"}
                </button>
                <button 
                    class={if *current_tab == "preview" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("preview".to_string())}
                >
                    {"👁️ Preview"}
                </button>
            </div>

            <div class="tab-content">
                {match (*current_tab).as_str() {
                    "admin" => html! {
                        <div class="theme-tab admin-theme-tab">
                            <div class="color-editor-layout">
                                <div class="color-editor">
                                    <div class="editor-header">
                                        <h3>{"Admin Theme Editor"}</h3>
                                        <div class="theme-controls">
                                            <div class="preset-controls">
                                                <select class="preset-dropdown" onchange={on_preset_change.clone()} value={(*selected_preset).clone()}>
                                                    {for (*saved_themes).iter().map(|theme| {
                                                        html! {
                                                            <option value={theme.clone()}>{theme.clone()}</option>
                                                        }
                                                    })}
                                                </select>
                                                <button class="preset-controls-button reset-button" onclick={reset_admin_defaults}>
                                                    {"Reset to Default"}
                                                </button>
                                            </div>
                                            <div class="save-controls">
                                                <input 
                                                    type="text" 
                                                    class="theme-name-input" 
                                                    placeholder="Theme name..."
                                                    value={(*theme_name_input).clone()}
                                                    onchange={on_theme_name_change.clone()}
                                                />
                                                <button class="save-controls-button save-theme-button" onclick={save_theme.clone()}>
                                                    {"Save Theme"}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="color-groups">
                                        <div class="color-group">
                                            <h4>{"Brand Colors"}</h4>
                                            <AdminColorInput 
                                                label="Primary" 
                                                value={admin_scheme.primary.clone()}
                                                property="primary"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Secondary" 
                                                value={admin_scheme.secondary.clone()}
                                                property="secondary"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Status Colors"}</h4>
                                            <AdminColorInput 
                                                label="Success" 
                                                value={admin_scheme.success.clone()}
                                                property="success"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Warning" 
                                                value={admin_scheme.warning.clone()}
                                                property="warning"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Danger" 
                                                value={admin_scheme.danger.clone()}
                                                property="danger"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Info" 
                                                value={admin_scheme.info.clone()}
                                                property="info"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Background & Surface"}</h4>
                                            <AdminColorInput 
                                                label="Background" 
                                                value={admin_scheme.background.clone()}
                                                property="background"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Surface" 
                                                value={admin_scheme.surface.clone()}
                                                property="surface"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Border" 
                                                value={admin_scheme.border.clone()}
                                                property="border"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Text Colors"}</h4>
                                            <AdminColorInput 
                                                label="Primary Text" 
                                                value={admin_scheme.text_primary.clone()}
                                                property="text_primary"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Secondary Text" 
                                                value={admin_scheme.text_secondary.clone()}
                                                property="text_secondary"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Header Styling"}</h4>
                                            <AdminColorInput 
                                                label="Header Text Color" 
                                                value={admin_scheme.header_text_color.clone()}
                                                property="header_text_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Header Border Color" 
                                                value={admin_scheme.header_border_color.clone()}
                                                property="header_border_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Header Shadow" 
                                                value={admin_scheme.header_shadow.clone()}
                                                property="header_shadow"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Sidebar Styling"}</h4>
                                            <AdminColorInput 
                                                label="Sidebar Border Color" 
                                                value={admin_scheme.sidebar_border_color.clone()}
                                                property="sidebar_border_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Sidebar Shadow" 
                                                value={admin_scheme.sidebar_shadow.clone()}
                                                property="sidebar_shadow"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Section Title Color" 
                                                value={admin_scheme.sidebar_section_title_color.clone()}
                                                property="sidebar_section_title_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Section Border Color" 
                                                value={admin_scheme.sidebar_section_border_color.clone()}
                                                property="sidebar_section_border_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Navigation Links"}</h4>
                                            <AdminColorInput 
                                                label="Link Text Color" 
                                                value={admin_scheme.nav_link_text_color.clone()}
                                                property="nav_link_text_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Link Hover Background" 
                                                value={admin_scheme.nav_link_hover_bg.clone()}
                                                property="nav_link_hover_bg"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Link Hover Text" 
                                                value={admin_scheme.nav_link_hover_text.clone()}
                                                property="nav_link_hover_text"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Active Link Shadow" 
                                                value={admin_scheme.nav_link_active_shadow.clone()}
                                                property="nav_link_active_shadow"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Public Navigation Links"}</h4>
                                            <AdminColorInput 
                                                label="Public Link Text" 
                                                value={admin_scheme.nav_link_public_text.clone()}
                                                property="nav_link_public_text"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Public Link Hover BG" 
                                                value={admin_scheme.nav_link_public_hover_bg.clone()}
                                                property="nav_link_public_hover_bg"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Public Link Hover Text" 
                                                value={admin_scheme.nav_link_public_hover_text.clone()}
                                                property="nav_link_public_hover_text"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Advanced Styling"}</h4>
                                            <AdminColorInput 
                                                label="Card Background" 
                                                value={admin_scheme.card_bg.clone()}
                                                property="card_bg"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Shadow Color" 
                                                value={admin_scheme.shadow_color.clone()}
                                                property="shadow_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                            <AdminColorInput 
                                                label="Accent Color" 
                                                value={admin_scheme.accent_color.clone()}
                                                property="accent_color"
                                                on_change={update_admin_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Gradients & Complex Styles"}</h4>
                                            <div class="color-input">
                                                <label>{"Header Gradient"}</label>
                                                <div class="color-input-group">
                                                    <input 
                                                        type="text" 
                                                        value={admin_scheme.header_gradient.clone()}
                                                        onchange={
                                                            let update_admin_color = update_admin_color.clone();
                                                            move |e: Event| {
                                                                let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                                                update_admin_color.emit(("header_gradient".to_string(), input.value()));
                                                            }
                                                        }
                                                        placeholder="linear-gradient(...)"
                                                        style="width: 100%; font-family: monospace; font-size: 0.8rem;"
                                                    />
                                                </div>
                                            </div>
                                            <div class="color-input">
                                                <label>{"Sidebar Background"}</label>
                                                <div class="color-input-group">
                                                    <input 
                                                        type="text" 
                                                        value={admin_scheme.sidebar_bg.clone()}
                                                        onchange={
                                                            let update_admin_color = update_admin_color.clone();
                                                            move |e: Event| {
                                                                let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                                                update_admin_color.emit(("sidebar_bg".to_string(), input.value()));
                                                            }
                                                        }
                                                        placeholder="linear-gradient(...)"
                                                        style="width: 100%; font-family: monospace; font-size: 0.8rem;"
                                                    />
                                                </div>
                                            </div>
                                            <div class="color-input">
                                                <label>{"Header Logo Gradient"}</label>
                                                <div class="color-input-group">
                                                    <input 
                                                        type="text" 
                                                        value={admin_scheme.header_logo_gradient.clone()}
                                                        onchange={
                                                            let update_admin_color = update_admin_color.clone();
                                                            move |e: Event| {
                                                                let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                                                update_admin_color.emit(("header_logo_gradient".to_string(), input.value()));
                                                            }
                                                        }
                                                        placeholder="linear-gradient(...)"
                                                        style="width: 100%; font-family: monospace; font-size: 0.8rem;"
                                                    />
                                                </div>
                                            </div>
                                            <div class="color-input">
                                                <label>{"Active Nav Background"}</label>
                                                <div class="color-input-group">
                                                    <input 
                                                        type="text" 
                                                        value={admin_scheme.nav_link_active_bg.clone()}
                                                        onchange={
                                                            let update_admin_color = update_admin_color.clone();
                                                            move |e: Event| {
                                                                let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                                                update_admin_color.emit(("nav_link_active_bg".to_string(), input.value()));
                                                            }
                                                        }
                                                        placeholder="linear-gradient(...)"
                                                        style="width: 100%; font-family: monospace; font-size: 0.8rem;"
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="apply-actions">
                                        <button class="save-controls-button save-theme-button" onclick={apply_admin_theme}>
                                            {"Apply Admin Theme"}
                                        </button>
                                    </div>
                                </div>

                                <div class="color-preview">
                                    <h3>{"Admin Preview"}</h3>
                                    <AdminPreview scheme={(*admin_scheme).clone()} />
                                </div>
                            </div>
                        </div>
                    },
                    "public" => html! {
                        <div class="theme-tab public-theme-tab">
                            <div class="public-theme-layout">
                                <div class="color-editor">
                                    <div class="editor-header">
                                        <h3>{"Public Theme Editor"}</h3>
                                        <div class="theme-controls">
                                            <div class="preset-controls">
                                                <select class="preset-dropdown" onchange={on_preset_change.clone()} value={(*selected_preset).clone()}>
                                                    {for (*saved_themes).iter().map(|theme| {
                                                        html! {
                                                            <option value={theme.clone()}>{theme.clone()}</option>
                                                        }
                                                    })}
                                                </select>
                                                <button class="preset-controls-button reset-button" onclick={reset_public_defaults}>
                                                    {"Reset to Default"}
                                                </button>
                                            </div>
                                            <div class="save-controls">
                                                <input 
                                                    type="text" 
                                                    class="theme-name-input" 
                                                    placeholder="Theme name..."
                                                    value={(*theme_name_input).clone()}
                                                    onchange={on_theme_name_change.clone()}
                                                />
                                                <button class="save-controls-button save-theme-button" onclick={save_theme.clone()}>
                                                    {"Save Theme"}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="color-groups">
                                        <div class="color-group">
                                            <h4>{"Primary Text Hierarchy"}</h4>
                                            <PublicColorInput 
                                                label="Primary Text" 
                                                value={public_scheme.text_primary.clone()}
                                                property="text_primary"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Secondary Text" 
                                                value={public_scheme.text_secondary.clone()}
                                                property="text_secondary"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Meta Text" 
                                                value={public_scheme.text_meta.clone()}
                                                property="text_meta"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Light Text" 
                                                value={public_scheme.text_light.clone()}
                                                property="text_light"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Muted Text" 
                                                value={public_scheme.text_muted.clone()}
                                                property="text_muted"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Heading Colors"}</h4>
                                            <PublicColorInput 
                                                label="Heading H1" 
                                                value={public_scheme.heading_h1.clone()}
                                                property="heading_h1"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Heading H2" 
                                                value={public_scheme.heading_h2.clone()}
                                                property="heading_h2"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Heading H3" 
                                                value={public_scheme.heading_h3.clone()}
                                                property="heading_h3"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Heading H4-H6" 
                                                value={public_scheme.heading_h4.clone()}
                                                property="heading_h4"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Link & Interactive Colors"}</h4>
                                            <PublicColorInput 
                                                label="Primary Links" 
                                                value={public_scheme.link_primary.clone()}
                                                property="link_primary"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Link Hover" 
                                                value={public_scheme.link_hover.clone()}
                                                property="link_hover"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Link Visited" 
                                                value={public_scheme.link_visited.clone()}
                                                property="link_visited"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Link Active" 
                                                value={public_scheme.link_active.clone()}
                                                property="link_active"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Header & Footer Text"}</h4>
                                            <PublicColorInput 
                                                label="Header Text" 
                                                value={public_scheme.header_text.clone()}
                                                property="header_text"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Header Text Hover" 
                                                value={public_scheme.header_text_hover.clone()}
                                                property="header_text_hover"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Footer Text" 
                                                value={public_scheme.footer_text.clone()}
                                                property="footer_text"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Footer Text Muted" 
                                                value={public_scheme.footer_text_muted.clone()}
                                                property="footer_text_muted"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Semantic Status Colors"}</h4>
                                            <PublicColorInput 
                                                label="Success" 
                                                value={public_scheme.success.clone()}
                                                property="success"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Warning" 
                                                value={public_scheme.warning.clone()}
                                                property="warning"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Error" 
                                                value={public_scheme.danger.clone()}
                                                property="danger"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Info" 
                                                value={public_scheme.info.clone()}
                                                property="info"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Layout & Background"}</h4>
                                            <PublicColorInput 
                                                label="Border Light" 
                                                value={public_scheme.border_light.clone()}
                                                property="border_light"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Background Light" 
                                                value={public_scheme.background_light.clone()}
                                                property="background_light"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Header Background" 
                                                value={public_scheme.header_bg.clone()}
                                                property="header_bg"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Footer Background" 
                                                value={public_scheme.footer_bg.clone()}
                                                property="footer_bg"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Hero Background" 
                                                value={public_scheme.hero_bg.clone()}
                                                property="hero_bg"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Card Shadow" 
                                                value={public_scheme.card_shadow.clone()}
                                                property="card_shadow"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>
                                    </div>

                                    <div class="apply-actions">
                                        <button class="save-controls-button save-theme-button" onclick={apply_public_theme}>
                                            {"Apply Public Theme"}
                                        </button>
                                    </div>
                                </div>
                                
                                <div class="color-preview-row">
                                    <h3>{"Public Preview"}</h3>
                                    <PublicPreview scheme={(*public_scheme).clone()} />
                                    
                                    <div class="public-text-preview">
                                        <h3>{"Text System Overview"}</h3>
                                        <div class="text-hierarchy-grid">
                                            <div class="text-column">
                                                <h4>{"Headings"}</h4>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--heading-h1"}</span>
                                                    <h1 style={format!("color: {}; font-size: 1.5rem; font-weight: 700; margin: 0.25rem 0;", public_scheme.heading_h1)}>{"Page Title"}</h1>
                                                </div>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--heading-h2"}</span>
                                                    <h2 style={format!("color: {}; font-size: 1.25rem; font-weight: 600; margin: 0.25rem 0;", public_scheme.heading_h2)}>{"Section"}</h2>
                                                </div>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--heading-h3"}</span>
                                                    <h3 style={format!("color: {}; font-size: 1.1rem; font-weight: 600; margin: 0.25rem 0;", public_scheme.heading_h3)}>{"Subsection"}</h3>
                                                </div>
                                            </div>
                                            
                                            <div class="text-column">
                                                <h4>{"Body Text"}</h4>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--text-primary"}</span>
                                                    <p style={format!("color: {}; margin: 0.25rem 0; font-weight: 600; font-size: 0.9rem;", public_scheme.text_primary)}>{"Primary content"}</p>
                                                </div>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--text-secondary"}</span>
                                                    <p style={format!("color: {}; margin: 0.25rem 0; font-size: 0.9rem;", public_scheme.text_secondary)}>{"Body paragraphs"}</p>
                                                </div>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--text-meta"}</span>
                                                    <p style={format!("color: {}; margin: 0.25rem 0; font-size: 0.8rem;", public_scheme.text_meta)}>{"Meta info"}</p>
                                                </div>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--text-light"}</span>
                                                    <p style={format!("color: {}; margin: 0.25rem 0; font-size: 0.75rem;", public_scheme.text_light)}>{"Captions"}</p>
                                                </div>
                                            </div>
                                            
                                            <div class="text-column">
                                                <h4>{"Interactive"}</h4>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--link-primary"}</span>
                                                    <a href="#" style={format!("color: {}; text-decoration: none; font-weight: 500; font-size: 0.9rem;", public_scheme.link_primary)}>{"Primary Link"}</a>
                                                </div>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--link-hover"}</span>
                                                    <a href="#" style={format!("color: {}; text-decoration: none; font-weight: 500; font-size: 0.9rem;", public_scheme.link_hover)}>{"Hover State"}</a>
                                                </div>
                                                <div class="text-sample-compact">
                                                    <span class="var-label">{"--link-visited"}</span>
                                                    <a href="#" style={format!("color: {}; text-decoration: none; font-weight: 500; font-size: 0.9rem;", public_scheme.link_visited)}>{"Visited"}</a>
                                                </div>
                                            </div>
                                            
                                            <div class="text-column">
                                                <h4>{"Semantic"}</h4>
                                                <div class="semantic-preview">
                                                    <div class="semantic-item" style={format!("color: {}; font-size: 0.8rem; margin: 0.25rem 0;", public_scheme.success)}>
                                                        {"✓ Success message"}
                                                    </div>
                                                    <div class="semantic-item" style={format!("color: {}; font-size: 0.8rem; margin: 0.25rem 0;", public_scheme.warning)}>
                                                        {"⚠ Warning message"}
                                                    </div>
                                                    <div class="semantic-item" style={format!("color: {}; font-size: 0.8rem; margin: 0.25rem 0;", public_scheme.danger)}>
                                                        {"✗ Error message"}
                                                    </div>
                                                    <div class="semantic-item" style={format!("color: {}; font-size: 0.8rem; margin: 0.25rem 0;", public_scheme.info)}>
                                                        {"ℹ Info message"}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="header-footer-preview">
                                            <h4>{"Navigation & Footer"}</h4>
                                            <div style={format!("background: #000; padding: 0.75rem; border-radius: 6px; margin: 0.5rem 0; display: flex; gap: 1rem; align-items: center;")}>
                                                <span style={format!("color: {}; font-weight: 500; font-size: 0.9rem;", public_scheme.header_text)}>{"Nav Item"}</span>
                                                <span style={format!("color: {}; font-weight: 500; font-size: 0.9rem;", public_scheme.header_text_hover)}>{"Nav Hover"}</span>
                                                <span style={format!("color: {}; font-size: 0.8rem;", public_scheme.footer_text_muted)}>{"Footer Text"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    "typography" => html! {
                        <div class="typography-tab">
                            <div class="typography-header">
                                <h3>{"📝 Typography System"}</h3>
                                <p>{"Consistent font hierarchy and spacing for optimal readability across all interfaces"}</p>
                            </div>
                            
                            <div class="typography-content">
                                <div class="typography-controls">
                                    <div class="font-family-selector">
                                        <label>{"Font Family"}</label>
                                        <select>
                                            <option value="system">{"System Font Stack"}</option>
                                            <option value="inter">{"Inter"}</option>
                                            <option value="roboto">{"Roboto"}</option>
                                            <option value="poppins">{"Poppins"}</option>
                                        </select>
                                    </div>
                                    <div class="base-size-slider">
                                        <label>{"Base Font Size"}</label>
                                        <input type="range" min="14" max="18" value="16" />
                                        <span>{"16px"}</span>
                                    </div>
                                    <div class="line-height-slider">
                                        <label>{"Line Height"}</label>
                                        <input type="range" min="1.2" max="1.8" step="0.1" value="1.6" />
                                        <span>{"1.6"}</span>
                                    </div>
                                </div>
                                
                                <div class="typography-preview">
                                    <div class="font-scale">
                                        <div class="font-example">
                                            <h1>{"Heading 1 - Main Page Title"}</h1>
                                            <div class="font-specs">{"48px | 1.2 line-height | 700 weight"}</div>
                                        </div>
                                        
                                        <div class="font-example">
                                            <h2>{"Heading 2 - Section Title"}</h2>
                                            <div class="font-specs">{"32px | 1.3 line-height | 600 weight"}</div>
                                        </div>
                                        
                                        <div class="font-example">
                                            <h3>{"Heading 3 - Subsection Title"}</h3>
                                            <div class="font-specs">{"24px | 1.4 line-height | 600 weight"}</div>
                                        </div>
                                        
                                        <div class="font-example">
                                            <h4>{"Heading 4 - Component Title"}</h4>
                                            <div class="font-specs">{"20px | 1.4 line-height | 500 weight"}</div>
                                        </div>
                                        
                                        <div class="font-example">
                                            <p>{"Body text - This is the main text used for content, articles, and general reading. It should be comfortable to read and provide good contrast against the background. Lorem ipsum dolor sit amet, consectetur adipiscing elit."}</p>
                                            <div class="font-specs">{"16px | 1.6 line-height | 400 weight"}</div>
                                        </div>
                                        
                                        <div class="font-example">
                                            <small>{"Small text - Used for captions, footnotes, timestamps, and secondary information that supplements the main content."}</small>
                                            <div class="font-specs">{"14px | 1.5 line-height | 400 weight"}</div>
                                        </div>
                                        
                                        <div class="font-example">
                                            <code>{"Code text - For inline code snippets and technical content"}</code>
                                            <div class="font-specs">{"14px | 1.4 line-height | Monospace"}</div>
                                        </div>
                                    </div>
                                    
                                    <div class="spacing-system">
                                        <h4>{"Spacing Scale"}</h4>
                                        <div class="spacing-examples">
                                            <div class="spacing-item">
                                                <div class="spacing-demo" style="height: 4px;"></div>
                                                <span>{"xs: 4px"}</span>
                                            </div>
                                            <div class="spacing-item">
                                                <div class="spacing-demo" style="height: 8px;"></div>
                                                <span>{"sm: 8px"}</span>
                                            </div>
                                            <div class="spacing-item">
                                                <div class="spacing-demo" style="height: 16px;"></div>
                                                <span>{"md: 16px"}</span>
                                            </div>
                                            <div class="spacing-item">
                                                <div class="spacing-demo" style="height: 24px;"></div>
                                                <span>{"lg: 24px"}</span>
                                            </div>
                                            <div class="spacing-item">
                                                <div class="spacing-demo" style="height: 32px;"></div>
                                                <span>{"xl: 32px"}</span>
                                            </div>
                                            <div class="spacing-item">
                                                <div class="spacing-demo" style="height: 48px;"></div>
                                                <span>{"2xl: 48px"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    "preview" => html! {
                        <div class="preview-tab">
                            <div class="preview-header">
                                <h3>{"👁️ Live Component Preview"}</h3>
                                <p>{"Real-time preview of how your theme choices affect UI components across admin and public interfaces"}</p>
                            </div>
                            
                            <div class="preview-grid">
                                <div class="preview-section admin-preview-section">
                                    <div class="section-header">
                                        <h4>{"🛠️ Admin Interface Components"}</h4>
                                        <div class="preview-actions">
                                            <button class="preview-action-btn" onclick={apply_admin_theme.clone()}>
                                                {"🎨 Apply Live"}
                                            </button>
                                        </div>
                                    </div>
                                    <div class="component-showcase">
                                        <AdminComponentPreview scheme={(*admin_scheme).clone()} />
                                    </div>
                                </div>
                                
                                <div class="preview-section public-preview-section">
                                    <div class="section-header">
                                        <h4>{"🌐 Public Site Components"}</h4>
                                        <div class="preview-actions">
                                            <button class="preview-action-btn" onclick={apply_public_theme.clone()}>
                                                {"🎨 Apply Live"}
                                            </button>
                                        </div>
                                    </div>
                                    <div class="component-showcase">
                                        <PublicComponentPreview scheme={(*public_scheme).clone()} />
                                    </div>
                                </div>
                            </div>
                            
                            <div class="preview-footer">
                                <div class="theme-comparison">
                                    <h4>{"Theme Comparison"}</h4>
                                    <div class="comparison-grid">
                                        <div class="theme-sample admin-sample">
                                            <div class="sample-header" style={format!("background: {}", admin_scheme.header_gradient)}>
                                                <h5>{"Admin Theme"}</h5>
                                            </div>
                                            <div class="sample-content" style={format!("background: {}; color: {}", admin_scheme.background, admin_scheme.text_primary)}>
                                                <div class="sample-colors">
                                                    <div class="color-dot" style={format!("background: {}", admin_scheme.primary)}></div>
                                                    <div class="color-dot" style={format!("background: {}", admin_scheme.success)}></div>
                                                    <div class="color-dot" style={format!("background: {}", admin_scheme.warning)}></div>
                                                    <div class="color-dot" style={format!("background: {}", admin_scheme.danger)}></div>
                                                </div>
                                                <p>{"Professional admin interface"}</p>
                                            </div>
                                        </div>
                                        
                                        <div class="theme-sample public-sample">
                                            <div class="sample-header" style={format!("background: {}; color: {}", public_scheme.text_primary, public_scheme.background_light)}>
                                                <h5>{"Public Theme"}</h5>
                                            </div>
                                            <div class="sample-content" style={format!("background: {}; color: {}", public_scheme.background_light, public_scheme.text_light)}>
                                                <div class="sample-colors">
                                                    <div class="color-dot" style={format!("background: {}", public_scheme.text_primary)}></div>
                                                    <div class="color-dot" style={format!("background: {}", public_scheme.success)}></div>
                                                    <div class="color-dot" style={format!("background: {}", public_scheme.warning)}></div>
                                                    <div class="color-dot" style={format!("background: {}", public_scheme.danger)}></div>
                                                </div>
                                                <p>{"Clean public website"}</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    },
                    _ => html! { <div>{"Invalid tab"}</div> }
                }}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AdminColorInputProps {
    label: String,
    value: String,
    property: String,
    on_change: Callback<(String, String)>,
}

#[function_component(AdminColorInput)]
fn admin_color_input(props: &AdminColorInputProps) -> Html {
    let on_change = {
        let property = props.property.clone();
        let callback = props.on_change.clone();
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            callback.emit((property.clone(), input.value()));
        })
    };

    html! {
        <div class="color-input">
            <label>{&props.label}</label>
            <div class="color-input-group">
                <input 
                    type="color" 
                    value={props.value.clone()}
                    onchange={on_change.clone()}
                />
                <input 
                    type="text" 
                    value={props.value.clone()}
                    onchange={on_change}
                    placeholder="#000000"
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PublicColorInputProps {
    label: String,
    value: String,
    property: String,
    on_change: Callback<(String, String)>,
}

#[function_component(PublicColorInput)]
fn public_color_input(props: &PublicColorInputProps) -> Html {
    let on_change = {
        let property = props.property.clone();
        let callback = props.on_change.clone();
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            callback.emit((property.clone(), input.value()));
        })
    };

    html! {
        <div class="color-input">
            <label>{&props.label}</label>
            <div class="color-input-group">
                <input 
                    type="color" 
                    value={props.value.clone()}
                    onchange={on_change.clone()}
                />
                <input 
                    type="text" 
                    value={props.value.clone()}
                    onchange={on_change}
                    placeholder="#000000"
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AdminPreviewProps {
    scheme: AdminColorScheme,
}

#[function_component(AdminPreview)]
fn admin_preview(props: &AdminPreviewProps) -> Html {
    html! {
        <div class="color-preview-container admin-preview" style={format!(
            "background: {}; color: {}; border: 1px solid {};",
            props.scheme.background, props.scheme.text_primary, props.scheme.border
        )}>
            <div class="preview-content">
                <div class="preview-header" style={format!(
                    "background: {}; padding: 1rem; margin: -1.5rem -1.5rem 1rem -1.5rem; border-radius: 8px 8px 0 0;",
                    props.scheme.header_gradient
                )}>
                    <h4 style="color: white; margin: 0;">{"Admin Panel Preview"}</h4>
                </div>
                <div class="preview-buttons">
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none; padding: 0.5rem 1rem; margin: 0.25rem; border-radius: 6px;",
                        props.scheme.primary
                    )}>{"Primary"}</button>
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none; padding: 0.5rem 1rem; margin: 0.25rem; border-radius: 6px;",
                        props.scheme.success
                    )}>{"Success"}</button>
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none; padding: 0.5rem 1rem; margin: 0.25rem; border-radius: 6px;",
                        props.scheme.warning
                    )}>{"Warning"}</button>
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none; padding: 0.5rem 1rem; margin: 0.25rem; border-radius: 6px;",
                        props.scheme.danger
                    )}>{"Danger"}</button>
                </div>
                <div class="preview-card" style={format!(
                    "background: {}; border: 1px solid {}; padding: 1rem; margin-top: 1rem; border-radius: 8px;",
                    props.scheme.surface, props.scheme.border
                )}>
                    <h5 style={format!("color: {}", props.scheme.text_primary)}>{"Admin Card Component"}</h5>
                    <p style={format!("color: {}", props.scheme.text_secondary)}>
                        {"This shows how admin interface components will appear with your theme."}
                    </p>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PublicPreviewProps {
    scheme: PublicColorScheme,
}

#[function_component(PublicPreview)]
fn public_preview(props: &PublicPreviewProps) -> Html {
    html! {
        <div class="color-preview-container public-preview" style={format!(
            "background: white; color: {}; border: 1px solid {};",
            props.scheme.text_primary, props.scheme.border_light
        )}>
            <div class="preview-content">
                <div class="preview-header" style={format!(
                    "background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-bottom: 1px solid {}; padding: 1rem; margin: -1.5rem -1.5rem 1rem -1.5rem;",
                    props.scheme.border_light
                )}>
                    <h4 style={format!("color: {}; margin: 0; font-weight: 700;", props.scheme.text_primary)}>{"Public Site Preview"}</h4>
                </div>
                <p style={format!("color: {}; margin-bottom: 1rem;", props.scheme.text_light)}>
                    {"This is how your public site will look with the minimalist design."}
                </p>
                <div class="preview-buttons">
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none; padding: 0.75rem 1.5rem; margin: 0.25rem; border-radius: 8px; font-weight: 500;",
                        props.scheme.text_primary
                    )}>{"Primary CTA"}</button>
                    <button class="btn" style={format!(
                        "background: transparent; color: {}; border: 1px solid {}; padding: 0.75rem 1.5rem; margin: 0.25rem; border-radius: 8px;",
                        props.scheme.text_primary, props.scheme.border_light
                    )}>{"Secondary"}</button>
                </div>
                <div class="preview-card" style={format!(
                    "background: {}; border: 1px solid {}; padding: 1.5rem; margin-top: 1rem; border-radius: 8px;",
                    props.scheme.background_light, props.scheme.border_light
                )}>
                    <h5 style={format!("color: {}", props.scheme.text_primary)}>{"Article Card"}</h5>
                    <p style={format!("color: {}; line-height: 1.6;", props.scheme.text_light)}>
                        {"This shows how content cards will appear on your public site."}
                    </p>
                    <small style={format!("color: {}", props.scheme.text_muted)}>
                        {"Published on January 1, 2024"}
                    </small>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AdminComponentPreviewProps {
    scheme: AdminColorScheme,
}

#[function_component(AdminComponentPreview)]
fn admin_component_preview(props: &AdminComponentPreviewProps) -> Html {
    html! {
        <div class="component-preview-grid admin-components">
            <div class="component-demo">
                <h4>{"Admin Buttons"}</h4>
                <div class="button-group">
                    <button class="save-controls-button save-theme-button" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 6px; margin: 0.25rem;", props.scheme.primary)}>{"Primary"}</button>
                    <button class="preset-controls-button reset-button" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 6px; margin: 0.25rem;", props.scheme.secondary)}>{"Secondary"}</button>
                    <button class="btn btn-success" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 6px; margin: 0.25rem;", props.scheme.success)}>{"Success"}</button>
                    <button class="btn btn-warning" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 6px; margin: 0.25rem;", props.scheme.warning)}>{"Warning"}</button>
                    <button class="btn btn-danger" style={format!("background: {}; color: white; border: none; padding: 0.5rem 1rem; border-radius: 6px; margin: 0.25rem;", props.scheme.danger)}>{"Danger"}</button>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Admin Forms"}</h4>
                <div class="form-group">
                    <label style={format!("color: {}; font-weight: 500; margin-bottom: 0.5rem; display: block;", props.scheme.text_primary)}>{"Text Input"}</label>
                    <input type="text" placeholder="Enter text here..." style={format!("border: 1px solid {}; background: {}; color: {}; padding: 0.75rem; border-radius: 6px; width: 100%;", props.scheme.border, props.scheme.background, props.scheme.text_primary)} />
                </div>
                <div class="form-group">
                    <label style={format!("color: {}; font-weight: 500; margin-bottom: 0.5rem; display: block;", props.scheme.text_primary)}>{"Select"}</label>
                    <select style={format!("border: 1px solid {}; background: {}; color: {}; padding: 0.75rem; border-radius: 6px; width: 100%;", props.scheme.border, props.scheme.background, props.scheme.text_primary)}>
                        <option>{"Option 1"}</option>
                        <option>{"Option 2"}</option>
                    </select>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Status Badges"}</h4>
                <div class="badge-group">
                    <span style={format!("background: {}; color: white; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; margin: 0.25rem;", props.scheme.success)}>{"Published"}</span>
                    <span style={format!("background: {}; color: white; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; margin: 0.25rem;", props.scheme.secondary)}>{"Draft"}</span>
                    <span style={format!("background: {}; color: white; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; margin: 0.25rem;", props.scheme.warning)}>{"Pending"}</span>
                    <span style={format!("background: {}; color: white; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 500; margin: 0.25rem;", props.scheme.info)}>{"Active"}</span>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Admin Cards"}</h4>
                <div class="card" style={format!("background: {}; border: 1px solid {}; border-radius: 8px; padding: 1rem;", props.scheme.surface, props.scheme.border)}>
                    <div class="card-header">
                        <h5 style={format!("color: {}; margin: 0 0 0.5rem 0; font-weight: 600;", props.scheme.text_primary)}>{"Dashboard Card"}</h5>
                    </div>
                    <p style={format!("color: {}; margin: 0; font-size: 0.875rem;", props.scheme.text_secondary)}>{"Admin interface components with professional styling."}</p>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PublicComponentPreviewProps {
    scheme: PublicColorScheme,
}

#[function_component(PublicComponentPreview)]
fn public_component_preview(props: &PublicComponentPreviewProps) -> Html {
    html! {
        <div class="component-preview-grid public-components">
            <div class="component-demo">
                <h4>{"Typography Hierarchy"}</h4>
                <div style="padding: 1rem; background: white; border-radius: 8px; border: 1px solid #eee;">
                    <h1 style={format!("color: {}; font-size: 2rem; font-weight: 700; margin: 0.5rem 0;", props.scheme.heading_h1)}>{"H1 Main Title"}</h1>
                    <h2 style={format!("color: {}; font-size: 1.5rem; font-weight: 600; margin: 0.5rem 0;", props.scheme.heading_h2)}>{"H2 Section Title"}</h2>
                    <h3 style={format!("color: {}; font-size: 1.25rem; font-weight: 600; margin: 0.5rem 0;", props.scheme.heading_h3)}>{"H3 Subsection"}</h3>
                    <p style={format!("color: {}; margin: 0.5rem 0; line-height: 1.6;", props.scheme.text_secondary)}>{"Body text for articles and content with proper readability."}</p>
                    <p style={format!("color: {}; font-size: 0.9rem; margin: 0.5rem 0;", props.scheme.text_meta)}>{"Meta information like dates and categories"}</p>
                    <small style={format!("color: {}; font-size: 0.8rem;", props.scheme.text_light)}>{"Caption and auxiliary text"}</small>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Interactive Links"}</h4>
                <div style="padding: 1rem; background: white; border-radius: 8px; border: 1px solid #eee;">
                    <p>
                        <a href="#" style={format!("color: {}; text-decoration: none; font-weight: 500; margin-right: 1rem;", props.scheme.link_primary)}>{"Primary Link"}</a>
                        <a href="#" style={format!("color: {}; text-decoration: none; font-weight: 500; margin-right: 1rem;", props.scheme.link_hover)}>{"Hover State"}</a>
                        <a href="#" style={format!("color: {}; text-decoration: none; font-weight: 500;", props.scheme.link_visited)}>{"Visited Link"}</a>
                    </p>
                    <div style="margin-top: 1rem;">
                        <span style={format!("background: {}; color: white; padding: 0.5rem 1rem; border-radius: 6px; font-weight: 500; margin-right: 0.5rem;", props.scheme.link_primary)}>{"Primary Button"}</span>
                        <span style={format!("background: transparent; color: {}; border: 1px solid {}; padding: 0.5rem 1rem; border-radius: 6px; font-weight: 500;", props.scheme.link_primary, props.scheme.border_light)}>{"Secondary Button"}</span>
                    </div>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Status Messages"}</h4>
                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    <div style={format!("background: #f0fdf4; border: 1px solid #bbf7d0; color: {}; padding: 0.75rem; border-radius: 6px; font-size: 0.875rem;", props.scheme.success)}>
                        {"✅ Success: Your changes have been saved successfully."}
                    </div>
                    <div style={format!("background: #fffbeb; border: 1px solid #fed7aa; color: {}; padding: 0.75rem; border-radius: 6px; font-size: 0.875rem;", props.scheme.warning)}>
                        {"⚠️ Warning: Please review your input before continuing."}
                    </div>
                    <div style={format!("background: #fef2f2; border: 1px solid #fecaca; color: {}; padding: 0.75rem; border-radius: 6px; font-size: 0.875rem;", props.scheme.danger)}>
                        {"❌ Error: Something went wrong. Please try again."}
                    </div>
                    <div style={format!("background: #f0f9ff; border: 1px solid #bae6fd; color: {}; padding: 0.75rem; border-radius: 6px; font-size: 0.875rem;", props.scheme.info)}>
                        {"ℹ️ Info: Here's some helpful information for you."}
                    </div>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Article Card Preview"}</h4>
                <div style={format!("background: {}; border: 1px solid {}; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px {};", props.scheme.background_light, props.scheme.border_light, props.scheme.card_shadow)}>
                    <div class="card-header">
                        <h3 style={format!("color: {}; margin: 0 0 0.75rem 0; font-weight: 700; font-size: 1.25rem;", props.scheme.heading_h3)}>{"Building Modern Web Apps"}</h3>
                    </div>
                    <p style={format!("color: {}; margin: 0 0 1rem 0; line-height: 1.6;", props.scheme.text_secondary)}>{"A comprehensive guide to building modern, responsive web applications using the latest technologies and best practices."}</p>
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <small style={format!("color: {}", props.scheme.text_meta)}>{"Published on January 15, 2024"}</small>
                        <a href="#" style={format!("color: {}; text-decoration: none; font-weight: 600; font-size: 0.875rem;", props.scheme.link_primary)}>{"Read More →"}</a>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Function to apply admin CSS variables dynamically
pub fn apply_admin_css_variables(scheme: &AdminColorScheme) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        // Override body background for admin interface by adding admin class
        if let Some(body) = document.body() {
            // Get existing class name and append admin-body
            let existing_class = body.class_name();
            let new_class = if existing_class.is_empty() {
                "admin-body".to_string()
            } else {
                format!("{} admin-body", existing_class)
            };
            body.set_class_name(&new_class);
        }
        
        if let Some(root) = document.document_element() {
            // Apply comprehensive admin-specific CSS variables
            let admin_vars = format!(
                "--admin-primary-color: {}; --admin-secondary-color: {}; --admin-success-color: {}; --admin-warning-color: {}; --admin-danger-color: {}; --admin-info-color: {}; --admin-bg-primary: {}; --admin-bg-secondary: {}; --admin-text-primary: {}; --admin-text-secondary: {}; --admin-border-color: {}; --admin-shadow-md: {}; --admin-accent-color: {}; --admin-header-gradient: {}; --admin-header-text-color: {}; --admin-header-border-color: {}; --admin-header-shadow: {}; --admin-header-text-shadow: {}; --admin-header-logo-gradient: {}; --admin-sidebar-bg: {}; --admin-sidebar-border-color: {}; --admin-sidebar-shadow: {}; --admin-sidebar-section-title-color: {}; --admin-sidebar-section-border-color: {}; --admin-nav-link-text-color: {}; --admin-nav-link-hover-bg: {}; --admin-nav-link-hover-text: {}; --admin-nav-link-active-bg: {}; --admin-nav-link-active-shadow: {}; --admin-nav-link-active-indicator: {}; --admin-nav-link-public-text: {}; --admin-nav-link-public-hover-bg: {}; --admin-nav-link-public-hover-text: {}; --admin-card-bg: {};",
                scheme.primary, scheme.secondary, scheme.success, scheme.warning, scheme.danger, scheme.info,
                scheme.surface, scheme.background, scheme.text_primary, scheme.text_secondary, scheme.border, scheme.shadow_color, scheme.accent_color,
                scheme.header_gradient, scheme.header_text_color, scheme.header_border_color, scheme.header_shadow, scheme.header_text_shadow, scheme.header_logo_gradient,
                scheme.sidebar_bg, scheme.sidebar_border_color, scheme.sidebar_shadow, scheme.sidebar_section_title_color, scheme.sidebar_section_border_color,
                scheme.nav_link_text_color, scheme.nav_link_hover_bg, scheme.nav_link_hover_text, scheme.nav_link_active_bg, scheme.nav_link_active_shadow, scheme.nav_link_active_indicator,
                scheme.nav_link_public_text, scheme.nav_link_public_hover_bg, scheme.nav_link_public_hover_text, scheme.card_bg
            );
            
            // Apply to root element with admin prefix only
            let current_style = root.get_attribute("style").unwrap_or_default();
            
            // Remove any existing admin variables to prevent conflicts
            let cleaned_style = current_style
                .split(';')
                .filter(|s| !s.trim().starts_with("--admin-"))
                .collect::<Vec<_>>()
                .join(";");
            
            let new_style = if cleaned_style.trim().is_empty() {
                admin_vars
            } else {
                format!("{}; {}", cleaned_style, admin_vars)
            };
            let _ = root.set_attribute("style", &new_style);
            
            // Update admin-specific styling with comprehensive style injection
            if let Some(style_element) = document.query_selector("style#admin-theme-overrides").ok().flatten() {
                let _ = style_element.remove();
            }
            
            if let Some(head) = document.head() {
                if let Ok(style_element) = document.create_element("style") {
                    style_element.set_id("admin-theme-overrides");
                    // Only refresh CSS variables - let the CSS cascade handle the rest
                    let css_overrides = format!(r#"
                        /* Refresh admin theme CSS variables only */
                        :root {{
                            --admin-primary-color: {} !important;
                            --admin-secondary-color: {} !important;
                            --admin-success-color: {} !important;
                            --admin-warning-color: {} !important;
                            --admin-danger-color: {} !important;
                            --admin-info-color: {} !important;
                            --admin-bg-primary: {} !important;
                            --admin-bg-secondary: {} !important;
                            --admin-text-primary: {} !important;
                            --admin-text-secondary: {} !important;
                            --admin-border-color: {} !important;
                            --admin-shadow-md: {} !important;
                            --admin-accent-color: {} !important;
                            --admin-header-gradient: {} !important;
                            --admin-header-text-color: {} !important;
                            --admin-header-border-color: {} !important;
                            --admin-header-shadow: {} !important;
                            --admin-header-text-shadow: {} !important;
                            --admin-header-logo-gradient: {} !important;
                            --admin-sidebar-bg: {} !important;
                            --admin-sidebar-border-color: {} !important;
                            --admin-sidebar-shadow: {} !important;
                            --admin-sidebar-section-title-color: {} !important;
                            --admin-sidebar-section-border-color: {} !important;
                            --admin-nav-link-text-color: {} !important;
                            --admin-nav-link-hover-bg: {} !important;
                            --admin-nav-link-hover-text: {} !important;
                            --admin-nav-link-active-bg: {} !important;
                            --admin-nav-link-active-shadow: {} !important;
                            --admin-nav-link-active-indicator: {} !important;
                            --admin-nav-link-public-text: {} !important;
                            --admin-nav-link-public-hover-bg: {} !important;
                            --admin-nav-link-public-hover-text: {} !important;
                            --admin-card-bg: {} !important;
                        }}
                    "#, 
                    scheme.primary, scheme.secondary, scheme.success, scheme.warning, scheme.danger, scheme.info,
                    scheme.surface, scheme.background, scheme.text_primary, scheme.text_secondary, scheme.border, scheme.shadow_color, scheme.accent_color,
                    scheme.header_gradient, scheme.header_text_color, scheme.header_border_color, scheme.header_shadow, scheme.header_text_shadow, scheme.header_logo_gradient,
                    scheme.sidebar_bg, scheme.sidebar_border_color, scheme.sidebar_shadow, scheme.sidebar_section_title_color, scheme.sidebar_section_border_color,
                    scheme.nav_link_text_color, scheme.nav_link_hover_bg, scheme.nav_link_hover_text, scheme.nav_link_active_bg, scheme.nav_link_active_shadow, scheme.nav_link_active_indicator,
                    scheme.nav_link_public_text, scheme.nav_link_public_hover_bg, scheme.nav_link_public_hover_text, scheme.card_bg
                    );
                    style_element.set_text_content(Some(&css_overrides));
                    let _ = head.append_child(&style_element);
                }
            }
            
            web_sys::console::log_1(&format!(
                "✅ Applied admin theme: {} - Variables updated in DOM", 
                scheme.name
            ).into());
        }
    }
}

// Function to apply public CSS variables dynamically  
pub fn apply_public_css_variables(scheme: &PublicColorScheme) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(root) = document.document_element() {
            // Apply comprehensive public-specific CSS variables
            let public_vars = format!(
                "--public-text-primary: {}; --public-text-secondary: {}; --public-text-meta: {}; --public-text-light: {}; --public-text-muted: {}; --public-link-primary: {}; --public-link-hover: {}; --public-link-visited: {}; --public-link-active: {}; --public-heading-h1: {}; --public-heading-h2: {}; --public-heading-h3: {}; --public-heading-h4: {}; --public-heading-h5: {}; --public-heading-h6: {}; --public-header-text: {}; --public-header-text-hover: {}; --public-footer-text: {}; --public-footer-text-muted: {}; --public-text-success: {}; --public-text-warning: {}; --public-text-error: {}; --public-text-info: {}; --public-border-light: {}; --public-background-light: {}; --public-header-bg: {}; --public-footer-bg: {}; --public-hero-bg: {}; --public-card-shadow: {};",
                scheme.text_primary, scheme.text_secondary, scheme.text_meta, scheme.text_light, scheme.text_muted,
                scheme.link_primary, scheme.link_hover, scheme.link_visited, scheme.link_active,
                scheme.heading_h1, scheme.heading_h2, scheme.heading_h3, scheme.heading_h4, scheme.heading_h5, scheme.heading_h6,
                scheme.header_text, scheme.header_text_hover, scheme.footer_text, scheme.footer_text_muted,
                scheme.success, scheme.warning, scheme.danger, scheme.info,
                scheme.border_light, scheme.background_light, scheme.header_bg, scheme.footer_bg, scheme.hero_bg, scheme.card_shadow
            );
            
            // Apply to root element with public prefix only
            let current_style = root.get_attribute("style").unwrap_or_default();
            
            // Remove any existing public variables to prevent conflicts
            let cleaned_style = current_style
                .split(';')
                .filter(|s| !s.trim().starts_with("--public-"))
                .collect::<Vec<_>>()
                .join(";");
            
            let new_style = if cleaned_style.trim().is_empty() {
                public_vars
            } else {
                format!("{}; {}", cleaned_style, public_vars)
            };
            let _ = root.set_attribute("style", &new_style);
            
            // Update public-specific styling with comprehensive style injection
            if let Some(style_element) = document.query_selector("style#public-theme-overrides").ok().flatten() {
                let _ = style_element.remove();
            }
            
            if let Some(head) = document.head() {
                if let Ok(style_element) = document.create_element("style") {
                    style_element.set_id("public-theme-overrides");
                    // Only refresh CSS variables - let the CSS cascade handle the rest
                    let css_overrides = format!(r#"
                        /* Refresh public theme CSS variables only */
                        :root {{
                            --public-text-primary: {} !important;
                            --public-text-secondary: {} !important;
                            --public-text-meta: {} !important;
                            --public-text-light: {} !important;
                            --public-text-muted: {} !important;
                            --public-link-primary: {} !important;
                            --public-link-hover: {} !important;
                            --public-link-visited: {} !important;
                            --public-link-active: {} !important;
                            --public-heading-h1: {} !important;
                            --public-heading-h2: {} !important;
                            --public-heading-h3: {} !important;
                            --public-heading-h4: {} !important;
                            --public-heading-h5: {} !important;
                            --public-heading-h6: {} !important;
                            --public-header-text: {} !important;
                            --public-header-text-hover: {} !important;
                            --public-footer-text: {} !important;
                            --public-footer-text-muted: {} !important;
                            --public-text-success: {} !important;
                            --public-text-warning: {} !important;
                            --public-text-error: {} !important;
                            --public-text-info: {} !important;
                            --public-border-light: {} !important;
                            --public-background-light: {} !important;
                            --public-header-bg: {} !important;
                            --public-footer-bg: {} !important;
                            --public-hero-bg: {} !important;
                            --public-card-shadow: {} !important;
                        }}
                    "#, 
                    scheme.text_primary,
                    scheme.text_secondary,
                    scheme.text_meta,
                    scheme.text_light,
                    scheme.text_muted,
                    scheme.link_primary,
                    scheme.link_hover,
                    scheme.link_visited,
                    scheme.link_active,
                    scheme.heading_h1,
                    scheme.heading_h2,
                    scheme.heading_h3,
                    scheme.heading_h4,
                    scheme.heading_h5,
                    scheme.heading_h6,
                    scheme.header_text,
                    scheme.header_text_hover,
                    scheme.footer_text,
                    scheme.footer_text_muted,
                    scheme.success,
                    scheme.warning,
                    scheme.danger,
                    scheme.info,
                    scheme.border_light,
                    scheme.background_light,
                    scheme.header_bg,
                    scheme.footer_bg,
                    scheme.hero_bg,
                    scheme.card_shadow
                    );
                    style_element.set_text_content(Some(&css_overrides));
                    let _ = head.append_child(&style_element);
                }
            }
            
            web_sys::console::log_1(&format!(
                "✅ Applied comprehensive public theme: {} - All text colors updated in DOM", 
                scheme.name
            ).into());
        }
    }
}