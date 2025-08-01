use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlElement};

#[derive(Debug, Clone, PartialEq)]
pub struct ColorScheme {
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
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
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
        }
    }
}

#[function_component(DesignSystemPage)]
pub fn design_system_page() -> Html {
    let active_scheme = use_state(|| ColorScheme::default());
    let admin_scheme = use_state(|| ColorScheme::default());
    let public_scheme = use_state(|| ColorScheme::default());
    let current_tab = use_state(|| "colors".to_string());

    // Initialize public scheme with different defaults
    if public_scheme.name == "Default" {
        public_scheme.set(ColorScheme {
            name: "Public Theme".to_string(),
            primary: "#000000".to_string(),
            secondary: "#333333".to_string(),
            success: "#059669".to_string(),
            warning: "#d97706".to_string(),
            danger: "#dc2626".to_string(),
            info: "#0891b2".to_string(),
            background: "#ffffff".to_string(),
            surface: "#f5f5f5".to_string(),
            text_primary: "#000000".to_string(),
            text_secondary: "#666666".to_string(),
            border: "#eeeeee".to_string(),
        });
    }

    let switch_tab = {
        let current_tab = current_tab.clone();
        Callback::from(move |tab: String| {
            current_tab.set(tab);
        })
    };

    let update_color = {
        let active_scheme = active_scheme.clone();
        Callback::from(move |(property, value): (String, String)| {
            let mut scheme = (*active_scheme).clone();
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
                _ => {}
            }
            active_scheme.set(scheme);
        })
    };

    let apply_to_admin = {
        let active_scheme = active_scheme.clone();
        let admin_scheme = admin_scheme.clone();
        Callback::from(move |_| {
            admin_scheme.set((*active_scheme).clone());
            apply_css_variables(&*active_scheme, "admin");
        })
    };

    let apply_to_public = {
        let active_scheme = active_scheme.clone();
        let public_scheme = public_scheme.clone();
        Callback::from(move |_| {
            public_scheme.set((*active_scheme).clone());
            apply_css_variables(&*active_scheme, "public");
        })
    };

    let reset_to_defaults = {
        let active_scheme = active_scheme.clone();
        Callback::from(move |_| {
            active_scheme.set(ColorScheme::default());
        })
    };

    let load_dark_preset = {
        let active_scheme = active_scheme.clone();
        Callback::from(move |_| {
            active_scheme.set(ColorScheme {
                name: "Dark Theme".to_string(),
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
            });
        })
    };

    html! {
        <div class="design-system-page">
            <div class="page-header">
                <h1>{"🎨 Design System"}</h1>
                <p>{"Manage themes and styling for both admin and public interfaces"}</p>
            </div>

            <div class="design-system-tabs">
                <button 
                    class={if *current_tab == "colors" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("colors".to_string())}
                >
                    {"Colors & Themes"}
                </button>
                <button 
                    class={if *current_tab == "typography" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("typography".to_string())}
                >
                    {"Typography"}
                </button>
                <button 
                    class={if *current_tab == "spacing" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("spacing".to_string())}
                >
                    {"Spacing & Layout"}
                </button>
                <button 
                    class={if *current_tab == "components" { "tab-button active" } else { "tab-button" }}
                    onclick={let cb = switch_tab.clone(); move |_| cb.emit("components".to_string())}
                >
                    {"Component Preview"}
                </button>
            </div>

            <div class="tab-content">
                {match (*current_tab).as_str() {
                    "colors" => html! {
                        <div class="colors-tab">
                            <div class="color-editor-layout">
                                <div class="color-editor">
                                    <div class="editor-header">
                                        <h3>{"Color Editor"}</h3>
                                        <div class="preset-buttons">
                                            <button class="btn btn-secondary" onclick={reset_to_defaults}>
                                                {"Reset to Default"}
                                            </button>
                                            <button class="btn btn-secondary" onclick={load_dark_preset}>
                                                {"Load Dark Preset"}
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="color-groups">
                                        <div class="color-group">
                                            <h4>{"Brand Colors"}</h4>
                                            <ColorInput 
                                                label="Primary" 
                                                value={active_scheme.primary.clone()}
                                                property="primary"
                                                on_change={update_color.clone()}
                                            />
                                            <ColorInput 
                                                label="Secondary" 
                                                value={active_scheme.secondary.clone()}
                                                property="secondary"
                                                on_change={update_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Status Colors"}</h4>
                                            <ColorInput 
                                                label="Success" 
                                                value={active_scheme.success.clone()}
                                                property="success"
                                                on_change={update_color.clone()}
                                            />
                                            <ColorInput 
                                                label="Warning" 
                                                value={active_scheme.warning.clone()}
                                                property="warning"
                                                on_change={update_color.clone()}
                                            />
                                            <ColorInput 
                                                label="Danger" 
                                                value={active_scheme.danger.clone()}
                                                property="danger"
                                                on_change={update_color.clone()}
                                            />
                                            <ColorInput 
                                                label="Info" 
                                                value={active_scheme.info.clone()}
                                                property="info"
                                                on_change={update_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Background & Surface"}</h4>
                                            <ColorInput 
                                                label="Background" 
                                                value={active_scheme.background.clone()}
                                                property="background"
                                                on_change={update_color.clone()}
                                            />
                                            <ColorInput 
                                                label="Surface" 
                                                value={active_scheme.surface.clone()}
                                                property="surface"
                                                on_change={update_color.clone()}
                                            />
                                            <ColorInput 
                                                label="Border" 
                                                value={active_scheme.border.clone()}
                                                property="border"
                                                on_change={update_color.clone()}
                                            />
                                        </div>

                                        <div class="color-group">
                                            <h4>{"Text Colors"}</h4>
                                            <ColorInput 
                                                label="Primary Text" 
                                                value={active_scheme.text_primary.clone()}
                                                property="text_primary"
                                                on_change={update_color.clone()}
                                            />
                                            <ColorInput 
                                                label="Secondary Text" 
                                                value={active_scheme.text_secondary.clone()}
                                                property="text_secondary"
                                                on_change={update_color.clone()}
                                            />
                                        </div>
                                    </div>

                                    <div class="apply-actions">
                                        <button class="btn btn-primary" onclick={apply_to_admin}>
                                            {"Apply to Admin Theme"}
                                        </button>
                                        <button class="btn btn-primary" onclick={apply_to_public}>
                                            {"Apply to Public Theme"}
                                        </button>
                                    </div>
                                </div>

                                <div class="color-preview">
                                    <h3>{"Live Preview"}</h3>
                                    <ColorPreview scheme={(*active_scheme).clone()} />
                                </div>
                            </div>

                            <div class="theme-status">
                                <div class="theme-card">
                                    <h4>{"Admin Theme"}</h4>
                                    <div class="theme-colors">
                                        <div class="color-swatch" style={format!("background-color: {}", admin_scheme.primary)}></div>
                                        <div class="color-swatch" style={format!("background-color: {}", admin_scheme.background)}></div>
                                        <div class="color-swatch" style={format!("background-color: {}", admin_scheme.text_primary)}></div>
                                    </div>
                                </div>
                                <div class="theme-card">
                                    <h4>{"Public Theme"}</h4>
                                    <div class="theme-colors">
                                        <div class="color-swatch" style={format!("background-color: {}", public_scheme.primary)}></div>
                                        <div class="color-swatch" style={format!("background-color: {}", public_scheme.background)}></div>
                                        <div class="color-swatch" style={format!("background-color: {}", public_scheme.text_primary)}></div>
                                    </div>
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
                    "spacing" => html! {
                        <div class="spacing-tab">
                            <h3>{"Spacing & Layout System"}</h3>
                            <div class="spacing-scale">
                                <div class="spacing-example" style="padding: 0.25rem; background: #f0f0f0; margin-bottom: 0.5rem;">
                                    {"XS - 0.25rem (4px)"}
                                </div>
                                <div class="spacing-example" style="padding: 0.5rem; background: #f0f0f0; margin-bottom: 0.5rem;">
                                    {"SM - 0.5rem (8px)"}
                                </div>
                                <div class="spacing-example" style="padding: 1rem; background: #f0f0f0; margin-bottom: 0.5rem;">
                                    {"MD - 1rem (16px)"}
                                </div>
                                <div class="spacing-example" style="padding: 1.5rem; background: #f0f0f0; margin-bottom: 0.5rem;">
                                    {"LG - 1.5rem (24px)"}
                                </div>
                                <div class="spacing-example" style="padding: 2rem; background: #f0f0f0; margin-bottom: 0.5rem;">
                                    {"XL - 2rem (32px)"}
                                </div>
                            </div>
                        </div>
                    },
                    "components" => html! {
                        <div class="components-tab">
                            <h3>{"Component Preview"}</h3>
                            <ComponentPreview scheme={(*active_scheme).clone()} />
                        </div>
                    },
                    _ => html! { <div>{"Invalid tab"}</div> }
                }}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ColorInputProps {
    label: String,
    value: String,
    property: String,
    on_change: Callback<(String, String)>,
}

#[function_component(ColorInput)]
fn color_input(props: &ColorInputProps) -> Html {
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
struct ColorPreviewProps {
    scheme: ColorScheme,
}

#[function_component(ColorPreview)]
fn color_preview(props: &ColorPreviewProps) -> Html {
    html! {
        <div class="color-preview-container" style={format!(
            "background: {}; color: {}; border: 1px solid {};",
            props.scheme.background, props.scheme.text_primary, props.scheme.border
        )}>
            <div class="preview-content">
                <h4 style={format!("color: {}", props.scheme.primary)}>{"Preview Content"}</h4>
                <p style={format!("color: {}", props.scheme.text_secondary)}>
                    {"This is how your theme will look with the current color settings."}
                </p>
                <div class="preview-buttons">
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none;",
                        props.scheme.primary
                    )}>{"Primary Button"}</button>
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none;",
                        props.scheme.success
                    )}>{"Success"}</button>
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none;",
                        props.scheme.warning
                    )}>{"Warning"}</button>
                    <button class="btn" style={format!(
                        "background: {}; color: white; border: none;",
                        props.scheme.danger
                    )}>{"Danger"}</button>
                </div>
                <div class="preview-card" style={format!(
                    "background: {}; border: 1px solid {}; padding: 1rem; margin-top: 1rem;",
                    props.scheme.surface, props.scheme.border
                )}>
                    <h5 style={format!("color: {}", props.scheme.text_primary)}>{"Card Component"}</h5>
                    <p style={format!("color: {}", props.scheme.text_secondary)}>
                        {"This is a preview of how cards and surfaces will appear."}
                    </p>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ComponentPreviewProps {
    scheme: ColorScheme,
}

#[function_component(ComponentPreview)]
fn component_preview(_props: &ComponentPreviewProps) -> Html {
    html! {
        <div class="component-preview-grid">
            <div class="component-demo">
                <h4>{"Buttons"}</h4>
                <div class="button-group">
                    <button class="btn btn-primary">{"Primary"}</button>
                    <button class="btn btn-secondary">{"Secondary"}</button>
                    <button class="btn btn-success">{"Success"}</button>
                    <button class="btn btn-warning">{"Warning"}</button>
                    <button class="btn btn-danger">{"Danger"}</button>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Form Elements"}</h4>
                <div class="form-group">
                    <label>{"Text Input"}</label>
                    <input type="text" placeholder="Enter text here..." />
                </div>
                <div class="form-group">
                    <label>{"Select"}</label>
                    <select>
                        <option>{"Option 1"}</option>
                        <option>{"Option 2"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>{"Textarea"}</label>
                    <textarea placeholder="Enter description..."></textarea>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Status Badges"}</h4>
                <div class="badge-group">
                    <span class="status-badge published">{"Published"}</span>
                    <span class="status-badge draft">{"Draft"}</span>
                    <span class="status-badge pending">{"Pending"}</span>
                    <span class="status-badge active">{"Active"}</span>
                </div>
            </div>

            <div class="component-demo">
                <h4>{"Cards"}</h4>
                <div class="card">
                    <div class="card-header">
                        <h5>{"Card Title"}</h5>
                    </div>
                    <p>{"This is a sample card component showing how content will be displayed with the current theme."}</p>
                </div>
            </div>
        </div>
    }
}

// Function to apply CSS variables dynamically (placeholder for now)
fn apply_css_variables(_scheme: &ColorScheme, _target: &str) {
    // TODO: Implement CSS variable setting using proper web APIs
    // This would require additional dependencies for CSS manipulation
    web_sys::console::log_1(&format!("Applied {} theme colors", _target).into());
}