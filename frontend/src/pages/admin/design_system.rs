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
    pub header_gradient: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PublicColorScheme {
    pub name: String,
    pub primary: String,
    pub text_light: String,
    pub text_lighter: String,
    pub border_light: String,
    pub background_light: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
    pub info: String,
}

impl Default for AdminColorScheme {
    fn default() -> Self {
        Self {
            name: "Admin Default".to_string(),
            primary: "#2563eb".to_string(),
            secondary: "#64748b".to_string(),
            success: "#059669".to_string(),
            warning: "#d97706".to_string(),
            danger: "#dc2626".to_string(),
            info: "#0891b2".to_string(),
            background: "#ffffff".to_string(),
            surface: "#f8fafc".to_string(),
            text_primary: "#1e293b".to_string(),
            text_secondary: "#64748b".to_string(),
            border: "#e2e8f0".to_string(),
            header_gradient: "linear-gradient(135deg, #4c1d95 0%, #312e81 50%, #1e1b4b 100%)".to_string(),
        }
    }
}

impl Default for PublicColorScheme {
    fn default() -> Self {
        Self {
            name: "Public Default".to_string(),
            primary: "#1a1a1a".to_string(),
            text_light: "#666666".to_string(),
            text_lighter: "#999999".to_string(),
            border_light: "#e5e5e5".to_string(),
            background_light: "#fafafa".to_string(),
            success: "#059669".to_string(),
            warning: "#d97706".to_string(),
            danger: "#dc2626".to_string(),
            info: "#0891b2".to_string(),
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
                "header_gradient" => scheme.header_gradient = value,
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
                "primary" => scheme.primary = value,
                "text_light" => scheme.text_light = value,
                "text_lighter" => scheme.text_lighter = value,
                "border_light" => scheme.border_light = value,
                "background_light" => scheme.background_light = value,
                "success" => scheme.success = value,
                "warning" => scheme.warning = value,
                "danger" => scheme.danger = value,
                "info" => scheme.info = value,
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

    let load_admin_dark_preset = {
        let admin_scheme = admin_scheme.clone();
        Callback::from(move |_: MouseEvent| {
            admin_scheme.set(AdminColorScheme {
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
            });
        })
    };

    let load_public_dark_preset = {
        let public_scheme = public_scheme.clone();
        Callback::from(move |_: MouseEvent| {
            public_scheme.set(PublicColorScheme {
                name: "Public Dark Theme".to_string(),
                primary: "#f8fafc".to_string(),
                text_light: "#cbd5e1".to_string(),
                text_lighter: "#94a3b8".to_string(),
                border_light: "#334155".to_string(),
                background_light: "#1e293b".to_string(),
                success: "#10b981".to_string(),
                warning: "#f59e0b".to_string(),
                danger: "#ef4444".to_string(),
                info: "#06b6d4".to_string(),
            });
        })
    };

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
            match ((*current_tab).as_str(), preset_name.as_str()) {
                ("admin", "Dark Preset") => {
                    admin_scheme.set(AdminColorScheme {
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
                    });
                },
                ("admin", "Light Preset") => {
                    admin_scheme.set(AdminColorScheme::default());
                },
                ("public", "Dark Preset") => {
                    public_scheme.set(PublicColorScheme {
                        name: "Public Dark Theme".to_string(),
                        primary: "#f8fafc".to_string(),
                        text_light: "#cbd5e1".to_string(),
                        text_lighter: "#94a3b8".to_string(),
                        border_light: "#334155".to_string(),
                        background_light: "#1e293b".to_string(),
                        success: "#10b981".to_string(),
                        warning: "#f59e0b".to_string(),
                        danger: "#ef4444".to_string(),
                        info: "#06b6d4".to_string(),
                    });
                },
                ("public", "Light Preset") => {
                    public_scheme.set(PublicColorScheme::default());
                },
                _ => {}
            }
        })
    };

    let save_theme = {
        let theme_name_input = theme_name_input.clone();
        let saved_themes = saved_themes.clone();
        let current_tab = current_tab.clone();
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
                            <div class="color-editor-layout">
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
                                            <h4>{"Brand Colors"}</h4>
                                            <PublicColorInput 
                                                label="Primary" 
                                                value={public_scheme.primary.clone()}
                                                property="primary"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Text Colors"}</h4>
                                            <PublicColorInput 
                                                label="Light Text" 
                                                value={public_scheme.text_light.clone()}
                                                property="text_light"
                                                on_change={update_public_color.clone()}
                                            />
                                            <PublicColorInput 
                                                label="Lighter Text" 
                                                value={public_scheme.text_lighter.clone()}
                                                property="text_lighter"
                                                on_change={update_public_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Layout Colors"}</h4>
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
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Status Colors"}</h4>
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
                                                label="Danger" 
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
                                    </div>

                                    <div class="apply-actions">
                                        <button class="save-controls-button save-theme-button" onclick={apply_public_theme}>
                                            {"Apply Public Theme"}
                                        </button>
                                    </div>
                                </div>

                                <div class="color-preview">
                                    <h3>{"Public Preview"}</h3>
                                    <PublicPreview scheme={(*public_scheme).clone()} />
                                </div>
                            </div>
                        </div>
                    },
                    "typography" => html! {
                        <div class="typography-tab">
                            <h3>{"Typography System"}</h3>
                            <div class="typography-preview">
                                <div class="font-scale">
                                    <h1>{"Heading 1 - Main Title"}</h1>
                                    <h2>{"Heading 2 - Section Title"}</h2>
                                    <h3>{"Heading 3 - Subsection"}</h3>
                                    <h4>{"Heading 4 - Component Title"}</h4>
                                    <p>{"Body text - Regular paragraph content for reading and general information."}</p>
                                    <small>{"Small text - For captions, footnotes, and secondary information."}</small>
                                </div>
                            </div>
                        </div>
                    },
                    "preview" => html! {
                        <div class="preview-tab">
                            <h3>{"Component Preview"}</h3>
                            <div class="preview-grid">
                                <div class="preview-section">
                                    <h4>{"Admin Components"}</h4>
                                    <AdminComponentPreview scheme={(*admin_scheme).clone()} />
                                </div>
                                <div class="preview-section">
                                    <h4>{"Public Components"}</h4>
                                    <PublicComponentPreview scheme={(*public_scheme).clone()} />
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
            props.scheme.primary, props.scheme.border_light
        )}>
            <div class="preview-content">
                <div class="preview-header" style={format!(
                    "background: rgba(255, 255, 255, 0.95); backdrop-filter: blur(10px); border-bottom: 1px solid {}; padding: 1rem; margin: -1.5rem -1.5rem 1rem -1.5rem;",
                    props.scheme.border_light
                )}>
                    <h4 style={format!("color: {}; margin: 0; font-weight: 700;", props.scheme.primary)}>{"Public Site Preview"}</h4>
                </div>
                <p style={format!("color: {}; margin-bottom: 1rem;", props.scheme.text_light)}>
                    {"This is how your public site will look with the minimalist design."}
                </p>
                <div class="preview-buttons">
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none; padding: 0.75rem 1.5rem; margin: 0.25rem; border-radius: 8px; font-weight: 500;",
                        props.scheme.primary
                    )}>{"Primary CTA"}</button>
                    <button class="btn" style={format!(
                        "background: transparent; color: {}; border: 1px solid {}; padding: 0.75rem 1.5rem; margin: 0.25rem; border-radius: 8px;",
                        props.scheme.primary, props.scheme.border_light
                    )}>{"Secondary"}</button>
                </div>
                <div class="preview-card" style={format!(
                    "background: {}; border: 1px solid {}; padding: 1.5rem; margin-top: 1rem; border-radius: 8px;",
                    props.scheme.background_light, props.scheme.border_light
                )}>
                    <h5 style={format!("color: {}", props.scheme.primary)}>{"Article Card"}</h5>
                    <p style={format!("color: {}; line-height: 1.6;", props.scheme.text_light)}>
                        {"This shows how content cards will appear on your public site."}
                    </p>
                    <small style={format!("color: {}", props.scheme.text_lighter)}>
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
                <h4>{"Public Buttons"}</h4>
                <div class="button-group">
                    <button class="save-controls-button save-theme-button" style={format!("background: {}; color: white; border: none; padding: 0.75rem 1.5rem; border-radius: 8px; margin: 0.25rem; font-weight: 500;", props.scheme.primary)}>{"Primary CTA"}</button>
                    <button class="preset-controls-button reset-button" style={format!("background: transparent; color: {}; border: 1px solid {}; padding: 0.75rem 1.5rem; border-radius: 8px; margin: 0.25rem;", props.scheme.primary, props.scheme.border_light)}>{"Secondary"}</button>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Contact Form"}</h4>
                <div class="form-group">
                    <label style={format!("color: {}; font-weight: 500; margin-bottom: 0.5rem; display: block;", props.scheme.primary)}>{"Email"}</label>
                    <input type="email" placeholder="your@email.com" style={format!("border: 1px solid {}; background: white; color: {}; padding: 0.75rem; border-radius: 8px; width: 100%;", props.scheme.border_light, props.scheme.primary)} />
                </div>
                <div class="form-group">
                    <label style={format!("color: {}; font-weight: 500; margin-bottom: 0.5rem; display: block;", props.scheme.primary)}>{"Message"}</label>
                    <textarea placeholder="Your message..." style={format!("border: 1px solid {}; background: white; color: {}; padding: 0.75rem; border-radius: 8px; width: 100%; height: 80px; resize: vertical;", props.scheme.border_light, props.scheme.primary)}></textarea>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Content Tags"}</h4>
                <div class="badge-group">
                    <span style={format!("background: {}; color: white; padding: 0.25rem 0.75rem; border-radius: 4px; font-size: 0.75rem; margin: 0.25rem;", props.scheme.primary)}>{"Technology"}</span>
                    <span style={format!("background: {}; color: white; padding: 0.25rem 0.75rem; border-radius: 4px; font-size: 0.75rem; margin: 0.25rem;", props.scheme.success)}>{"Featured"}</span>
                    <span style={format!("background: {}; color: white; padding: 0.25rem 0.75rem; border-radius: 4px; font-size: 0.75rem; margin: 0.25rem;", props.scheme.info)}>{"Tutorial"}</span>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Article Card"}</h4>
                <div class="card" style={format!("background: {}; border: 1px solid {}; border-radius: 8px; padding: 1.5rem;", props.scheme.background_light, props.scheme.border_light)}>
                    <div class="card-header">
                        <h5 style={format!("color: {}; margin: 0 0 0.75rem 0; font-weight: 700;", props.scheme.primary)}>{"Getting Started with Rust"}</h5>
                    </div>
                    <p style={format!("color: {}; margin: 0 0 1rem 0; line-height: 1.6;", props.scheme.text_light)}>{"Learn the fundamentals of Rust programming with this comprehensive guide."}</p>
                    <small style={format!("color: {}", props.scheme.text_lighter)}>{"Published on January 15, 2024"}</small>
                </div>
            </div>
        </div>
    }
}

// Function to apply admin CSS variables dynamically
fn apply_admin_css_variables(scheme: &AdminColorScheme) {
    web_sys::console::log_1(&format!(
        "Applied admin theme: {} - Primary: {}, Background: {}, Header: {}", 
        scheme.name, scheme.primary, scheme.background, scheme.header_gradient
    ).into());
}

// Function to apply public CSS variables dynamically  
fn apply_public_css_variables(scheme: &PublicColorScheme) {
    web_sys::console::log_1(&format!(
        "Applied public theme: {} - Primary: {}, Text Light: {}, Background: {}", 
        scheme.name, scheme.primary, scheme.text_light, scheme.background_light
    ).into());
}