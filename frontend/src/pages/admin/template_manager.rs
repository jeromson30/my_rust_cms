use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::services::navigation_service::{MenuArea, ComponentTemplate, NavigationItem, get_menu_areas, get_component_templates, update_menu_area, update_component_template, get_navigation_by_area};

#[derive(Clone, PartialEq)]
pub enum TemplateView {
    MenuAreas,
    ComponentTemplates,
    ContainerSettings,
}

#[derive(Clone, PartialEq)]
pub struct MenuAreaState {
    pub area_name: String,
    pub display_name: String,
    pub is_active: bool,
    pub mobile_behavior: Option<String>,
    pub hamburger_icon: Option<String>,
    pub settings: serde_json::Value,
}

impl Default for MenuAreaState {
    fn default() -> Self {
        Self {
            area_name: String::new(),
            display_name: String::new(),
            is_active: false,
            mobile_behavior: None,
            hamburger_icon: None,
            settings: serde_json::json!({}),
        }
    }
}

#[function_component(TemplateManager)]
pub fn template_manager() -> Html {
    let current_view = use_state(|| TemplateView::MenuAreas);
    let menu_areas = use_state(Vec::<MenuArea>::new);

    let component_templates = use_state(Vec::<ComponentTemplate>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load initial data
    {
        let menu_areas = menu_areas.clone();
        let component_templates = component_templates.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                
                // Load template data in parallel
                let areas_result = get_menu_areas().await;
                let components_result = get_component_templates().await;
                
                match (areas_result, components_result) {
                    (Ok(areas), Ok(components)) => {
                        menu_areas.set(areas);
                        component_templates.set(components);
                        loading.set(false);
                    }
                    _ => {
                        error.set(Some("Failed to load template data".to_string()));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    let switch_to_areas = {
        let current_view = current_view.clone();
        Callback::from(move |_| current_view.set(TemplateView::MenuAreas))
    };

    let switch_to_components = {
        let current_view = current_view.clone();
        Callback::from(move |_| current_view.set(TemplateView::ComponentTemplates))
    };

    let switch_to_container = {
        let current_view = current_view.clone();
        Callback::from(move |_| current_view.set(TemplateView::ContainerSettings))
    };

    html! {
        <div class="template-manager">
            <div class="page-header">
                <h1>{"🎨 Template Manager"}</h1>
                <p>{"Configure menu areas, component templates, and global container settings"}</p>
            </div>

            {if let Some(ref error_msg) = *error {
                html! {
                    <div class="error-message">
                        <strong>{"Error: "}</strong>{error_msg}
                    </div>
                }
            } else {
                html! {}
            }}

            <div class="template-tabs">
                <button 
                    class={if *current_view == TemplateView::MenuAreas { "tab-button active" } else { "tab-button" }}
                    onclick={switch_to_areas}
                >
                    {"📍 Menu Areas"}
                </button>
                <button 
                    class={if *current_view == TemplateView::ComponentTemplates { "tab-button active" } else { "tab-button" }}
                    onclick={switch_to_components}
                >
                    {"🧩 Component Templates"}
                </button>
                <button 
                    class={if *current_view == TemplateView::ContainerSettings { "tab-button active" } else { "tab-button" }}
                    onclick={switch_to_container}
                >
                    {"📦 Container Settings"}
                </button>
            </div>

            <div class="template-content">
                {if *loading {
                    html! {
                        <div class="loading">
                            <div class="loading-spinner"></div>
                            <p>{"Loading template configuration..."}</p>
                        </div>
                    }
                } else {
                    match (*current_view).clone() {
                        TemplateView::MenuAreas => html! { 
                            <MenuAreasView 
                                menu_areas={(*menu_areas).clone()}
                                on_toggle={{
                                    let menu_areas = menu_areas.clone();
                                    let error = error.clone();
                                    Callback::from(move |(area_name, is_active): (String, bool)| {
                                        web_sys::console::log_1(&format!("Toggle {} to {}", area_name, is_active).into());
                                        
                                        // Update local state immediately for responsive UI
                                        let mut areas = (*menu_areas).clone();
                                        if let Some(area) = areas.iter_mut().find(|a| a.area_name == area_name) {
                                            area.is_active = is_active;
                                            let updated_area = area.clone(); // Clone before setting state
                                            menu_areas.set(areas.clone());
                                            
                                            // Persist to backend
                                            let menu_areas_clone = menu_areas.clone();
                                            let error_clone = error.clone();
                                            let area_name_clone = area_name.clone();
                                            wasm_bindgen_futures::spawn_local(async move {
                                                match update_menu_area(&area_name_clone, &updated_area).await {
                                                    Ok(_) => {
                                                        web_sys::console::log_1(&format!("Successfully updated {} area", area_name_clone).into());
                                                    }
                                                    Err(e) => {
                                                        error_clone.set(Some(format!("Failed to update {}: {:?}", area_name_clone, e)));
                                                        // Revert local state on error
                                                        let mut reverted_areas = areas;
                                                        if let Some(revert_area) = reverted_areas.iter_mut().find(|a| a.area_name == area_name_clone) {
                                                            revert_area.is_active = !is_active;
                                                            menu_areas_clone.set(reverted_areas);
                                                        }
                                                    }
                                                }
                                            });
                                        } else {
                                            // Handle standard areas that might not exist in backend yet
                                            error.set(Some(format!("Area '{}' not found in backend", area_name)));
                                        }
                                    })
                                }}
                            /> 
                        },
                        TemplateView::ComponentTemplates => html! { 
                            <ComponentTemplatesView 
                                component_templates={(*component_templates).clone()}
                                on_modify={Callback::noop()}
                            /> 
                        },
                        TemplateView::ContainerSettings => html! { <ContainerSettingsView /> },
                    }
                }}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MenuAreasViewProps {
    pub menu_areas: Vec<MenuArea>,
    pub on_toggle: Callback<(String, bool)>,
}

#[function_component(MenuAreasView)]
pub fn menu_areas_view(props: &MenuAreasViewProps) -> Html {
    let get_area_info = |area_name: &str| -> (String, String, String, bool) {
        if let Some(area) = props.menu_areas.iter().find(|a| a.area_name == area_name) {
            (
                area.display_name.clone(),
                format!("Status: {}", if area.is_active { "Active" } else { "Inactive" }),
                if area.is_active { "area-status active" } else { "area-status inactive" }.to_string(),
                area.is_active
            )
        } else {
            // Default data for standard areas
            match area_name {
                "header" => ("Header Menu".to_string(), "Status: Active".to_string(), "area-status active".to_string(), true),
                "footer" => ("Footer Menu".to_string(), "Status: Active".to_string(), "area-status active".to_string(), true),
                "floating" => ("Floating Menu".to_string(), "Status: Disabled".to_string(), "area-status inactive".to_string(), false),
                _ => ("Unknown".to_string(), "Status: Unknown".to_string(), "area-status".to_string(), false)
            }
        }
    };

    let handle_toggle = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |(area_name, is_active): (String, bool)| {
            // For now, just call the callback - the parent will handle the actual API call
            on_toggle.emit((area_name, is_active));
        })
    };

    html! {
        <div class="menu-areas-section">
            <h2>{"Menu Areas Configuration"}</h2>
            <p>{"Enable or disable different menu areas for your site"}</p>
            
            <div class="area-cards">
                <div class="area-card">
                    <div class="area-header">
                        <h3>{"📱 Header Menu"}</h3>
                        <div class="area-toggle">
                            <label class="toggle-switch">
                                <input 
                                    type="checkbox"
                                    checked={get_area_info("header").3}
                                    onchange={
                                        let handle_toggle = handle_toggle.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            handle_toggle.emit(("header".to_string(), target.checked()));
                                        })
                                    }
                                />
                                <span class="slider"></span>
                            </label>
                        </div>
                    </div>
                    <p>{"Main navigation with mobile hamburger support"}</p>
                    <div class={get_area_info("header").2}>{get_area_info("header").1}</div>
                </div>
                
                <div class="area-card">
                    <div class="area-header">
                        <h3>{"🦶 Footer Menu"}</h3>
                        <div class="area-toggle">
                            <label class="toggle-switch">
                                <input 
                                    type="checkbox"
                                    checked={get_area_info("footer").3}
                                    onchange={
                                        let handle_toggle = handle_toggle.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            handle_toggle.emit(("footer".to_string(), target.checked()));
                                        })
                                    }
                                />
                                <span class="slider"></span>
                            </label>
                        </div>
                    </div>
                    <p>{"Footer navigation with layout options"}</p>
                    <div class={get_area_info("footer").2}>{get_area_info("footer").1}</div>
                </div>
                
                <div class="area-card">
                    <div class="area-header">
                        <h3>{"🎈 Floating Menu"}</h3>
                        <div class="area-toggle">
                            <label class="toggle-switch">
                                <input 
                                    type="checkbox"
                                    checked={get_area_info("floating").3}
                                    onchange={
                                        let handle_toggle = handle_toggle.clone();
                                        Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            handle_toggle.emit(("floating".to_string(), target.checked()));
                                        })
                                    }
                                />
                                <span class="slider"></span>
                            </label>
                        </div>
                    </div>
                    <p>{"Floating navigation elements"}</p>
                    <div class={get_area_info("floating").2}>{get_area_info("floating").1}</div>
                </div>

                // Show custom menu areas
                {for props.menu_areas.iter().filter(|area| area.area_name.starts_with("custom_")).map(|area| {
                    html! {
                        <div class="area-card">
                            <div class="area-header">
                                <h3>{format!("🧩 {}", area.display_name)}</h3>
                                <div class="area-toggle">
                                    <label class="toggle-switch">
                                        <input 
                                            type="checkbox"
                                            checked={area.is_active}
                                            onchange={
                                                let handle_toggle = handle_toggle.clone();
                                                let area_name = area.area_name.clone();
                                                Callback::from(move |e: Event| {
                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                    handle_toggle.emit((area_name.clone(), target.checked()));
                                                })
                                            }
                                        />
                                        <span class="slider"></span>
                                    </label>
                                </div>
                            </div>
                            <p>{"Custom menu for page builder integration"}</p>
                            <div class={if area.is_active { "area-status active" } else { "area-status inactive" }}>
                                {format!("Status: {}", if area.is_active { "Active" } else { "Inactive" })}
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}



#[derive(Properties, PartialEq)]
pub struct ComponentTemplatesViewProps {
    pub component_templates: Vec<ComponentTemplate>,
    pub on_modify: Callback<(String, String)>, // (component_id, property)
}

#[function_component(ComponentTemplatesView)]
pub fn component_templates_view(props: &ComponentTemplatesViewProps) -> Html {
    let editing_component = use_state(|| None::<String>);
    let editing_template = use_state(|| None::<ComponentTemplate>);
    let saving = use_state(|| false);
    let save_error = use_state(|| None::<String>);
    
    // Navigation items for live previews
    let header_navigation = use_state(Vec::<NavigationItem>::new);
    let footer_navigation = use_state(Vec::<NavigationItem>::new);

    // Load navigation items for previews
    {
        let header_navigation = header_navigation.clone();
        let footer_navigation = footer_navigation.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Load header navigation
                if let Ok(header_items) = get_navigation_by_area("header").await {
                    header_navigation.set(header_items);
                }
                
                // Load footer navigation  
                if let Ok(footer_items) = get_navigation_by_area("footer").await {
                    footer_navigation.set(footer_items);
                }
            });
            || ()
        }, ());
    }

    let close_editor = {
        let editing_component = editing_component.clone();
        let editing_template = editing_template.clone();
        let save_error = save_error.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            editing_component.set(None);
            editing_template.set(None);
            save_error.set(None);
        })
    };

    let save_template = {
        let editing_template = editing_template.clone();
        let saving = saving.clone();
        let save_error = save_error.clone();
        let close_editor = close_editor.clone();
        Callback::from(move |_| {
            if let Some(template) = (*editing_template).clone() {
                saving.set(true);
                save_error.set(None);
                let saving_clone = saving.clone();
                let save_error_clone = save_error.clone();
                let close_editor_clone = close_editor.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    match update_component_template(template.id, &template).await {
                        Ok(_) => {
                            web_sys::console::log_1(&"Component template saved successfully".into());
                            saving_clone.set(false);
                            // Close editor programmatically - create a dummy mouse event
                            if let Some(window) = web_sys::window() {
                                if let Ok(event) = web_sys::MouseEvent::new("click") {
                                    close_editor_clone.emit(event);
                                }
                            }
                        }
                        Err(e) => {
                            save_error_clone.set(Some(format!("Failed to save template: {:?}", e)));
                            saving_clone.set(false);
                        }
                    }
                });
            }
        })
    };

    // Helper function to update template data
    let update_template_data = {
        let editing_template = editing_template.clone();
        Callback::from(move |(key, value): (String, serde_json::Value)| {
            if let Some(mut template) = (*editing_template).clone() {
                if let Some(data) = template.template_data.as_object_mut() {
                    data.insert(key, value);
                    editing_template.set(Some(template));
                }
            }
        })
    };

    html! {
        <div class="component-templates-section">
            <h2>{"Component Templates"}</h2>
            <p>{"Manage templates for major layout components with live styling"}</p>
            
            {if let Some(ref component_id) = *editing_component {
                html! {
                    <div class="editor-modal">
                        <div class="editor-overlay" onclick={close_editor.clone()}></div>
                        <div class="editor-panel">
                            <div class="editor-header">
                                <h3>{format!("Edit {} Component", component_id.replace("_", " "))}</h3>
                                <button class="close-btn" onclick={close_editor.clone()}>{"×"}</button>
                            </div>
                            <div class="editor-content">
                                <div class="editor-sidebar">
                                    {match component_id.as_str() {
                                        "header" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Header Layout"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Position"}</label>
                                                        <select 
                                                            class="property-select"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("position".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        >
                                                            <option value="static">{"Static"}</option>
                                                            <option value="sticky" selected=true>{"Sticky"}</option>
                                                            <option value="fixed">{"Fixed"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Container Width"}</label>
                                                        <select class="property-select">
                                                            <option value="full">{"Full Width"}</option>
                                                            <option value="contained" selected=true>{"Container (1200px)"}</option>
                                                            <option value="fluid">{"Fluid"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Height"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={{
                                                                if let Some(template) = (*editing_template).as_ref() {
                                                                    template.template_data.get("height")
                                                                        .and_then(|v| v.as_str())
                                                                        .unwrap_or("80px")
                                                                        .to_string()
                                                                } else {
                                                                    "80px".to_string()
                                                                }
                                                            }}
                                                            class="property-input"
                                                            onchange={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: Event| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("height".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                            onblur={{
                                                                let update_template_data = update_template_data.clone();
                                                                Callback::from(move |e: FocusEvent| {
                                                                    if let Some(target) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                                                        let value = target.value();
                                                                        update_template_data.emit(("height".to_string(), serde_json::Value::String(value)));
                                                                    }
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Z-Index"}</label>
                                                        <input type="number" value="1000" class="property-input" />
                                                    </div>
                                                </div>
                                                
                                                <div class="property-group">
                                                    <h4>{"Navigation Style"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Navigation Layout"}</label>
                                                        <select class="property-select">
                                                            <option value="horizontal" selected=true>{"Horizontal"}</option>
                                                            <option value="centered">{"Centered"}</option>
                                                            <option value="split">{"Logo Left / Menu Right"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Item Spacing"}</label>
                                                        <input type="text" value="2rem" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Hover Effect"}</label>
                                                        <select class="property-select">
                                                            <option value="underline" selected=true>{"Underline"}</option>
                                                            <option value="background">{"Background Color"}</option>
                                                            <option value="scale">{"Scale"}</option>
                                                            <option value="none">{"None"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Logo Settings"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Logo Type"}</label>
                                                        <select class="property-select">
                                                            <option value="text" selected=true>{"Text Logo"}</option>
                                                            <option value="image">{"Image Logo"}</option>
                                                            <option value="icon">{"Icon + Text"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Logo Size"}</label>
                                                        <input type="text" value="1.5rem" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Logo Font Weight"}</label>
                                                        <select class="property-select">
                                                            <option value="400">{"Normal"}</option>
                                                            <option value="600" selected=true>{"Semi-Bold"}</option>
                                                            <option value="700">{"Bold"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Mobile Behavior"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Mobile Menu Style"}</label>
                                                        <select class="property-select">
                                                            <option value="hamburger" selected=true>{"Hamburger Menu"}</option>
                                                            <option value="dots">{"Three Dots"}</option>
                                                            <option value="hidden">{"Hide Menu"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Mobile Breakpoint"}</label>
                                                        <input type="text" value="768px" class="property-input" />
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        "footer" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Footer Layout"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Footer Style"}</label>
                                                        <select class="property-select">
                                                            <option value="simple" selected=true>{"Simple"}</option>
                                                            <option value="multi-column">{"Multi-Column"}</option>
                                                            <option value="centered">{"Centered"}</option>
                                                            <option value="minimal">{"Minimal"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Container Width"}</label>
                                                        <select class="property-select">
                                                            <option value="full" selected=true>{"Full Width"}</option>
                                                            <option value="contained">{"Container (1200px)"}</option>
                                                            <option value="fluid">{"Fluid"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Vertical Padding"}</label>
                                                        <input type="text" value="3rem 0" class="property-input" />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Footer Navigation"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Navigation Layout"}</label>
                                                        <select class="property-select">
                                                            <option value="horizontal" selected=true>{"Horizontal"}</option>
                                                            <option value="vertical">{"Vertical"}</option>
                                                            <option value="grid">{"Grid (2x2)"}</option>
                                                            <option value="hidden">{"Hidden"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Link Spacing"}</label>
                                                        <input type="text" value="1.5rem" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Separator"}</label>
                                                        <select class="property-select">
                                                            <option value="none" selected=true>{"None"}</option>
                                                            <option value="pipe">{"Pipe (|)"}</option>
                                                            <option value="dot">{"Dot (•)"}</option>
                                                            <option value="line">{"Divider Line"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Copyright & Text"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Copyright Position"}</label>
                                                        <select class="property-select">
                                                            <option value="center" selected=true>{"Center"}</option>
                                                            <option value="left">{"Left"}</option>
                                                            <option value="right">{"Right"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Copyright Text"}</label>
                                                        <input type="text" value="© 2024 My Rust CMS" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Additional Text"}</label>
                                                        <input type="text" value="Built with Rust & Yew" class="property-input" />
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        "sidebar" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Sidebar Layout"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Position"}</label>
                                                        <select class="property-select">
                                                            <option value="left">{"Left Side"}</option>
                                                            <option value="right" selected=true>{"Right Side"}</option>
                                                            <option value="both">{"Both Sides"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Width"}</label>
                                                        <input type="text" value="300px" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Sticky Behavior"}</label>
                                                        <select class="property-select">
                                                            <option value="none">{"Normal Flow"}</option>
                                                            <option value="sticky" selected=true>{"Sticky"}</option>
                                                            <option value="fixed">{"Fixed Position"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Mobile Behavior"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Mobile Display"}</label>
                                                        <select class="property-select">
                                                            <option value="hidden" selected=true>{"Hidden"}</option>
                                                            <option value="bottom">{"Move to Bottom"}</option>
                                                            <option value="drawer">{"Slide-out Drawer"}</option>
                                                            <option value="accordion">{"Collapsible"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Mobile Breakpoint"}</label>
                                                        <input type="text" value="768px" class="property-input" />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Content Sections"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Default Sections"}</label>
                                                        <div class="checkbox-list">
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" checked=true />
                                                                <span>{"Navigation Links"}</span>
                                                            </label>
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" checked=true />
                                                                <span>{"Recent Posts"}</span>
                                                            </label>
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" />
                                                                <span>{"Categories"}</span>
                                                            </label>
                                                            <label class="checkbox-item">
                                                                <input type="checkbox" />
                                                                <span>{"Archives"}</span>
                                                            </label>
                                                        </div>
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        "modal" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Modal Behavior"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Animation Style"}</label>
                                                        <select class="property-select">
                                                            <option value="fade" selected=true>{"Fade In/Out"}</option>
                                                            <option value="scale">{"Scale In/Out"}</option>
                                                            <option value="slide-down">{"Slide Down"}</option>
                                                            <option value="slide-up">{"Slide Up"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Animation Duration"}</label>
                                                        <input type="text" value="300ms" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Close on Backdrop Click"}</label>
                                                        <select class="property-select">
                                                            <option value="true" selected=true>{"Yes"}</option>
                                                            <option value="false">{"No"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Modal Sizing"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Default Size"}</label>
                                                        <select class="property-select">
                                                            <option value="small">{"Small (400px)"}</option>
                                                            <option value="medium" selected=true>{"Medium (600px)"}</option>
                                                            <option value="large">{"Large (800px)"}</option>
                                                            <option value="xl">{"Extra Large (1000px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Max Height"}</label>
                                                        <input type="text" value="90vh" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Mobile Behavior"}</label>
                                                        <select class="property-select">
                                                            <option value="responsive" selected=true>{"Responsive"}</option>
                                                            <option value="fullscreen">{"Full Screen"}</option>
                                                            <option value="bottom-sheet">{"Bottom Sheet"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Backdrop & Overlay"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Backdrop Blur"}</label>
                                                        <input type="range" min="0" max="10" value="4" class="property-slider" />
                                                        <span class="slider-value">{"4px"}</span>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Backdrop Opacity"}</label>
                                                        <input type="range" min="0" max="100" value="50" class="property-slider" />
                                                        <span class="slider-value">{"50%"}</span>
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        "main_container" => html! {
                                            <>
                                                <div class="property-group">
                                                    <h4>{"Container Layout"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Layout System"}</label>
                                                        <select class="property-select">
                                                            <option value="css-grid" selected=true>{"CSS Grid"}</option>
                                                            <option value="flexbox">{"Flexbox"}</option>
                                                            <option value="float">{"Float (Legacy)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Max Width"}</label>
                                                        <input type="text" value="1200px" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Horizontal Padding"}</label>
                                                        <input type="text" value="2rem" class="property-input" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Gap Between Sections"}</label>
                                                        <input type="text" value="2rem" class="property-input" />
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Grid Configuration"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Main Content Area"}</label>
                                                        <input type="text" value="1fr" class="property-input" placeholder="e.g., 1fr, 800px" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Sidebar Area"}</label>
                                                        <input type="text" value="300px" class="property-input" placeholder="e.g., 300px, 25%" />
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Grid Template"}</label>
                                                        <select class="property-select">
                                                            <option value="content-sidebar" selected=true>{"Content + Sidebar"}</option>
                                                            <option value="sidebar-content">{"Sidebar + Content"}</option>
                                                            <option value="content-only">{"Content Only"}</option>
                                                            <option value="three-column">{"Three Column"}</option>
                                                        </select>
                                                    </div>
                                                </div>

                                                <div class="property-group">
                                                    <h4>{"Responsive Behavior"}</h4>
                                                    <div class="property-item">
                                                        <label>{"Mobile Layout"}</label>
                                                        <select class="property-select">
                                                            <option value="stack" selected=true>{"Stack Vertically"}</option>
                                                            <option value="hide-sidebar">{"Hide Sidebar"}</option>
                                                            <option value="drawer">{"Sidebar as Drawer"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-item">
                                                        <label>{"Mobile Padding"}</label>
                                                        <input type="text" value="1rem" class="property-input" />
                                                    </div>
                                                </div>
                                            </>
                                        },
                                        _ => html! {
                                            <div class="property-group">
                                                <h4>{"Component Properties"}</h4>
                                                <p>{"Select a component to customize its properties."}</p>
                                            </div>
                                        }
                                    }}
                                </div>
                                
                                <div class="editor-preview">
                                    <h4>{"Live Preview"}</h4>
                                    <div class="preview-container">
                                        {match component_id.as_str() {
                                            "header" => html! {
                                                <div class="preview-header">
                                                    <div class="preview-nav">
                                                        <span class="preview-logo">{"🏠 My Site"}</span>
                                                        <div class="preview-menu">
                                                            {
                                                                if (*header_navigation).is_empty() {
                                                                    html! {
                                                                        <>
                                                                            <span class="preview-item">{"Home"}</span>
                                                                            <span class="preview-item">{"Posts"}</span>
                                                                            <span class="preview-item">{"About"}</span>
                                                                        </>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <>
                                                                            {for (*header_navigation).iter().take(5).map(|item| {
                                                                                html! { <span class="preview-item">{&item.title}</span> }
                                                                            })}
                                                                        </>
                                                                    }
                                                                }
                                                            }
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            "footer" => html! {
                                                <div class="preview-footer">
                                                    <div class="preview-footer-content">
                                                        <div class="preview-footer-nav">
                                                            {
                                                                if (*footer_navigation).is_empty() {
                                                                    html! {
                                                                        <>
                                                                            <span class="preview-footer-item">{"Privacy"}</span>
                                                                            <span class="preview-footer-item">{"Terms"}</span>
                                                                            <span class="preview-footer-item">{"Contact"}</span>
                                                                        </>
                                                                    }
                                                                } else {
                                                                    html! {
                                                                        <>
                                                                            {for (*footer_navigation).iter().take(4).map(|item| {
                                                                                html! { <span class="preview-footer-item">{&item.title}</span> }
                                                                            })}
                                                                        </>
                                                                    }
                                                                }
                                                            }
                                                        </div>
                                                        <p class="preview-footer-text">{"© 2024 My Rust CMS - Built with Rust & Yew"}</p>
                                                    </div>
                                                </div>
                                            },
                                            _ => html! {
                                                <div class="preview-placeholder">
                                                    <p>{format!("{} Component Preview", component_id)}</p>
                                                </div>
                                            }
                                        }}
                                    </div>
                                </div>
                            </div>
                            
                            {if let Some(ref error_msg) = *save_error {
                                html! {
                                    <div class="error-message" style="margin: 1rem;">
                                        <strong>{"Error: "}</strong>{error_msg}
                                    </div>
                                }
                            } else {
                                html! {}
                            }}

                            <div class="editor-actions">
                                <button 
                                    class="btn-primary" 
                                    onclick={save_template.clone()}
                                    disabled={*saving}
                                >
                                    {if *saving { "Saving..." } else { "Save Changes" }}
                                </button>
                                <button class="btn-secondary">{"Reset to Default"}</button>
                                <button class="btn-secondary" onclick={close_editor.clone()}>{"Cancel"}</button>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            
            <div class="template-component-grid">
                // Header Component Template
                <div class="component-card primary">
                    <div class="component-preview">
                        <div class="live-header-preview">
                            <div class="live-nav-bar">
                                <span class="live-logo">{"🏠 My Site"}</span>
                                <div class="live-nav-items">
                                    {
                                        if (*header_navigation).is_empty() {
                                            html! {
                                                <>
                                                    <span>{"Home"}</span>
                                                    <span>{"Posts"}</span>
                                                    <span>{"About"}</span>
                                                </>
                                            }
                                        } else {
                                            html! {
                                                <>
                                                    {for (*header_navigation).iter().take(4).map(|item| {
                                                        html! { <span>{&item.title}</span> }
                                                    })}
                                                </>
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="component-info">
                        <h4>{"🎯 Header Component"}</h4>
                        <p>{"Main site header with navigation, logo, and mobile responsive design"}</p>
                        <div class="component-properties">
                            <span class="property-tag active">{"Active"}</span>
                            <span class="property-tag">{"Sticky Position"}</span>
                            <span class="property-tag">{"1200px Max"}</span>
                            <span class="property-tag">{"Mobile Responsive"}</span>
                        </div>
                        <button 
                            class="btn-primary"
                            onclick={{
                                let editing_component = editing_component.clone();
                                let editing_template = editing_template.clone();
                                let props_templates = props.component_templates.clone();
                                Callback::from(move |_| {
                                    editing_component.set(Some("header".to_string()));
                                    // Find and load the header template
                                    if let Some(template) = props_templates.iter().find(|t| t.component_type == "header") {
                                        editing_template.set(Some(template.clone()));
                                    }
                                })
                            }}
                        >
                            {"Customize Header"}
                        </button>
                    </div>
                </div>

                // Footer Component Template
                <div class="component-card primary">
                    <div class="component-preview">
                        <div class="live-footer-preview">
                            <div class="live-footer-content">
                                <div class="live-footer-nav">
                                    {
                                        if (*footer_navigation).is_empty() {
                                            html! {
                                                <>
                                                    <span>{"Privacy"}</span>
                                                    <span>{"Terms"}</span>
                                                    <span>{"Contact"}</span>
                                                </>
                                            }
                                        } else {
                                            html! {
                                                <>
                                                    {for (*footer_navigation).iter().take(4).map(|item| {
                                                        html! { <span>{&item.title}</span> }
                                                    })}
                                                </>
                                            }
                                        }
                                    }
                                </div>
                                <p>{"© 2024 My Rust CMS - Built with Rust & Yew"}</p>
                            </div>
                        </div>
                    </div>
                    <div class="component-info">
                        <h4>{"📍 Footer Component"}</h4>
                        <p>{"Site footer with navigation links, copyright, and customizable layout options"}</p>
                        <div class="component-properties">
                            <span class="property-tag active">{"Active"}</span>
                            <span class="property-tag">{"Full Width"}</span>
                            <span class="property-tag">{"Horizontal Layout"}</span>
                            <span class="property-tag">{"Responsive"}</span>
                        </div>
                        <button 
                            class="btn-primary"
                            onclick={{
                                let editing_component = editing_component.clone();
                                let editing_template = editing_template.clone();
                                let props_templates = props.component_templates.clone();
                                Callback::from(move |_| {
                                    editing_component.set(Some("footer".to_string()));
                                    if let Some(template) = props_templates.iter().find(|t| t.component_type == "footer") {
                                        editing_template.set(Some(template.clone()));
                                    }
                                })
                            }}
                        >
                            {"Customize Footer"}
                        </button>
                    </div>
                </div>

                // Sidebar Component Template
                <div class="component-card secondary">
                    <div class="component-preview">
                        <div class="live-sidebar-preview">
                            <div class="sidebar-header">{"📋 Sidebar"}</div>
                            <div class="sidebar-items">
                                <div class="sidebar-item">{"Navigation"}</div>
                                <div class="sidebar-item">{"Recent Posts"}</div>
                                <div class="sidebar-item">{"Categories"}</div>
                                <div class="sidebar-item">{"Archives"}</div>
                            </div>
                        </div>
                    </div>
                    <div class="component-info">
                        <h4>{"📋 Sidebar Component"}</h4>
                        <p>{"Configurable sidebar for additional navigation, widgets, and content areas"}</p>
                        <div class="component-properties">
                            <span class="property-tag inactive">{"Optional"}</span>
                            <span class="property-tag">{"250px Width"}</span>
                            <span class="property-tag">{"Left/Right Position"}</span>
                            <span class="property-tag">{"Collapsible"}</span>
                        </div>
                        <button 
                            class="btn-primary"
                            onclick={{
                                let editing_component = editing_component.clone();
                                let editing_template = editing_template.clone();
                                let props_templates = props.component_templates.clone();
                                Callback::from(move |_| {
                                    editing_component.set(Some("sidebar".to_string()));
                                    if let Some(template) = props_templates.iter().find(|t| t.component_type == "sidebar") {
                                        editing_template.set(Some(template.clone()));
                                    }
                                })
                            }}
                        >
                            {"Customize Sidebar"}
                        </button>
                    </div>
                </div>

                // Modal Component Template
                <div class="component-card secondary">
                    <div class="component-preview">
                        <div class="live-modal-preview">
                            <div class="modal-backdrop">
                                <div class="modal-content">
                                    <div class="modal-header">
                                        <span>{"Modal Title"}</span>
                                        <span class="modal-close">{"×"}</span>
                                    </div>
                                    <div class="modal-body">{"Content area"}</div>
                                    <div class="modal-footer">
                                        <button class="modal-btn">{"Cancel"}</button>
                                        <button class="modal-btn primary">{"Confirm"}</button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="component-info">
                        <h4>{"🪟 Modal Component"}</h4>
                        <p>{"Overlay modals for forms, dialogs, and interactive content with backdrop styling"}</p>
                        <div class="component-properties">
                            <span class="property-tag active">{"Active"}</span>
                            <span class="property-tag">{"Center Positioned"}</span>
                            <span class="property-tag">{"Backdrop Blur"}</span>
                            <span class="property-tag">{"Animation"}</span>
                        </div>
                        <button 
                            class="btn-primary"
                            onclick={{
                                let editing_component = editing_component.clone();
                                let editing_template = editing_template.clone();
                                let props_templates = props.component_templates.clone();
                                Callback::from(move |_| {
                                    editing_component.set(Some("modal".to_string()));
                                    if let Some(template) = props_templates.iter().find(|t| t.component_type == "modal") {
                                        editing_template.set(Some(template.clone()));
                                    }
                                })
                            }}
                        >
                            {"Customize Modal"}
                        </button>
                    </div>
                </div>

                // Main Container Component Template
                <div class="component-card primary">
                    <div class="component-preview">
                        <div class="live-container-preview">
                            <div class="container-outline">
                                <div class="container-header">{"Header"}</div>
                                <div class="container-main">
                                    <div class="container-content">{"Main Content Area"}</div>
                                    <div class="container-sidebar">{"Sidebar"}</div>
                                </div>
                                <div class="container-footer">{"Footer"}</div>
                            </div>
                        </div>
                    </div>
                    <div class="component-info">
                        <h4>{"📦 Main Container"}</h4>
                        <p>{"Primary layout container that wraps all content with responsive grid system"}</p>
                        <div class="component-properties">
                            <span class="property-tag active">{"Active"}</span>
                            <span class="property-tag">{"1200px Max"}</span>
                            <span class="property-tag">{"CSS Grid"}</span>
                            <span class="property-tag">{"Responsive"}</span>
                        </div>
                        <button 
                            class="btn-primary"
                            onclick={{
                                let editing_component = editing_component.clone();
                                let editing_template = editing_template.clone();
                                let props_templates = props.component_templates.clone();
                                Callback::from(move |_| {
                                    editing_component.set(Some("main_container".to_string()));
                                    if let Some(template) = props_templates.iter().find(|t| t.component_type == "main_container") {
                                        editing_template.set(Some(template.clone()));
                                    }
                                })
                            }}
                        >
                            {"Customize Container"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(ContainerSettingsView)]
pub fn container_settings_view() -> Html {
    html! {
        <div class="container-settings-section">
            <h2>{"📦 Container Settings"}</h2>
            <p>{"Manage global breakpoints, typography, and container settings that tie into the design system"}</p>
            
            <div class="settings-grid">
                <div class="settings-card">
                    <h3>{"📐 Global Breakpoints"}</h3>
                    <p>{"Define responsive breakpoints for the entire site"}</p>
                    
                    <div class="breakpoint-list">
                        <div class="breakpoint-item">
                            <label>{"Mobile (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input type="text" value="768px" class="breakpoint-input" />
                                <span class="breakpoint-preview">{"< 768px"}</span>
                            </div>
                        </div>
                        <div class="breakpoint-item">
                            <label>{"Tablet (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input type="text" value="1024px" class="breakpoint-input" />
                                <span class="breakpoint-preview">{"768px - 1024px"}</span>
                            </div>
                        </div>
                        <div class="breakpoint-item">
                            <label>{"Desktop (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input type="text" value="1200px" class="breakpoint-input" />
                                <span class="breakpoint-preview">{"1024px - 1200px"}</span>
                            </div>
                        </div>
                        <div class="breakpoint-item">
                            <label>{"Wide Screen (max-width)"}</label>
                            <div class="breakpoint-input-group">
                                <input type="text" value="1440px" class="breakpoint-input" />
                                <span class="breakpoint-preview">{"> 1200px"}</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="settings-card">
                    <h3>{"🎯 Typography System"}</h3>
                    <p>{"Configure global typography that integrates with the design system"}</p>
                    
                    <div class="typography-settings">
                        <div class="typography-group">
                            <h4>{"Base Settings"}</h4>
                            <div class="typography-item">
                                <label>{"Base Font Size"}</label>
                                <select class="typography-select">
                                    <option value="14px">{"14px (Small)"}</option>
                                    <option value="16px" selected=true>{"16px (Standard)"}</option>
                                    <option value="18px">{"18px (Large)"}</option>
                                </select>
                            </div>
                            <div class="typography-item">
                                <label>{"Scale Ratio"}</label>
                                <select class="typography-select">
                                    <option value="1.125">{"1.125 (Minor Second)"}</option>
                                    <option value="1.25" selected=true>{"1.25 (Major Third)"}</option>
                                    <option value="1.5">{"1.5 (Perfect Fifth)"}</option>
                                    <option value="1.618">{"1.618 (Golden Ratio)"}</option>
                                </select>
                            </div>
                            <div class="typography-item">
                                <label>{"Line Height"}</label>
                                <input type="number" value="1.5" step="0.1" min="1" max="2" class="typography-input" />
                            </div>
                        </div>
                        
                        <div class="typography-preview">
                            <h4>{"Typography Preview"}</h4>
                            <div class="preview-text">
                                <h1 class="preview-h1">{"Heading 1 - Main Title"}</h1>
                                <h2 class="preview-h2">{"Heading 2 - Section Title"}</h2>
                                <p class="preview-paragraph">
                                    {"This is a paragraph demonstrating the body text with current typography settings."}
                                </p>
                                <code class="preview-code">{"console.log('Code example');"}</code>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="settings-card">
                    <h3>{"📦 Container Configuration"}</h3>
                    <p>{"Configure the main container that wraps site content"}</p>
                    
                    <div class="container-settings">
                        <div class="container-group">
                            <h4>{"Container Width"}</h4>
                            <div class="container-item">
                                <label>{"Width Type"}</label>
                                <select class="container-select">
                                    <option value="fixed" selected=true>{"Fixed Width"}</option>
                                    <option value="fluid">{"Fluid Width"}</option>
                                    <option value="hybrid">{"Hybrid"}</option>
                                </select>
                            </div>
                            <div class="container-item">
                                <label>{"Max Width"}</label>
                                <input type="text" value="1200px" class="container-input" />
                            </div>
                            <div class="container-item">
                                <label>{"Horizontal Padding"}</label>
                                <input type="text" value="1rem" class="container-input" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="settings-actions">
                <button class="btn-primary large">{"Save Global Settings"}</button>
                <button class="btn-secondary">{"Reset to Defaults"}</button>
                <button class="btn-secondary">{"Export Settings"}</button>
            </div>
        </div>
    }
}