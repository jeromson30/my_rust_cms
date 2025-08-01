use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use pulldown_cmark::{Parser, html};

#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProps {
    pub value: String,
    pub on_change: Callback<String>,
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let show_preview = use_state(|| false);
    let textarea_ref = use_node_ref();

    let on_input = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            on_change.emit(target.value());
        })
    };

    let toggle_preview = {
        let show_preview = show_preview.clone();
        Callback::from(move |_| {
            show_preview.set(!*show_preview);
        })
    };

    let insert_text = {
        let textarea_ref = textarea_ref.clone();
        let on_change = props.on_change.clone();
        let value = props.value.clone();
        
        Callback::from(move |text: String| {
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                let start = textarea.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;
                let end = textarea.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize;
                
                let mut new_value = value.clone();
                new_value.replace_range(start..end, &text);
                
                on_change.emit(new_value);
                
                // Set cursor position after inserted text
                let new_pos = (start + text.len()) as u32;
                let _ = textarea.set_selection_start(Some(new_pos));
                let _ = textarea.set_selection_end(Some(new_pos));
                let _ = textarea.focus();
            }
        })
    };

    let on_bold = {
        let insert_text = insert_text.clone();
        Callback::from(move |_| {
            insert_text.emit("**bold text**".to_string());
        })
    };

    let on_italic = {
        let insert_text = insert_text.clone();
        Callback::from(move |_| {
            insert_text.emit("*italic text*".to_string());
        })
    };

    let on_header = {
        let insert_text = insert_text.clone();
        Callback::from(move |_| {
            insert_text.emit("## Header".to_string());
        })
    };

    let on_link = {
        let insert_text = insert_text.clone();
        Callback::from(move |_| {
            insert_text.emit("[link text](https://example.com)".to_string());
        })
    };

    let on_list = {
        let insert_text = insert_text.clone();
        Callback::from(move |_| {
            insert_text.emit("- List item".to_string());
        })
    };

    let on_code = {
        let insert_text = insert_text.clone();
        Callback::from(move |_| {
            insert_text.emit("`code`".to_string());
        })
    };

    // Convert markdown to HTML for preview
    let html_content = if *show_preview {
        let parser = Parser::new(&props.value);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    } else {
        String::new()
    };

    html! {
        <div class="markdown-editor">
            <div class="markdown-toolbar">
                <div class="toolbar-group">
                    <button type="button" class="toolbar-btn" onclick={on_bold} title="Bold">
                        <strong>{"B"}</strong>
                    </button>
                    <button type="button" class="toolbar-btn" onclick={on_italic} title="Italic">
                        <em>{"I"}</em>
                    </button>
                    <button type="button" class="toolbar-btn" onclick={on_header} title="Header">
                        {"H"}
                    </button>
                </div>
                
                <div class="toolbar-group">
                    <button type="button" class="toolbar-btn" onclick={on_link} title="Link">
                        {"🔗"}
                    </button>
                    <button type="button" class="toolbar-btn" onclick={on_list} title="List">
                        {"• List"}
                    </button>
                    <button type="button" class="toolbar-btn" onclick={on_code} title="Code">
                        {"<>"}
                    </button>
                </div>
                
                <div class="toolbar-group">
                    <button 
                        type="button" 
                        class={if *show_preview { "toolbar-btn active" } else { "toolbar-btn" }}
                        onclick={toggle_preview}
                        title="Toggle Preview"
                    >
                        {if *show_preview { "📝 Edit" } else { "👁 Preview" }}
                    </button>
                </div>
            </div>

            <div class="markdown-content">
                if *show_preview {
                    <div class="markdown-preview">
                        <div class="preview-content">
                            {Html::from_html_unchecked(html_content.into())}
                        </div>
                    </div>
                } else {
                    <textarea
                        ref={textarea_ref}
                        class="markdown-textarea"
                        value={props.value.clone()}
                        oninput={on_input}
                        placeholder={props.placeholder.clone().unwrap_or_else(|| "Write your content in Markdown...".to_string())}
                        rows={props.rows.unwrap_or(15).to_string()}
                    />
                }
            </div>
            
            <div class="markdown-help">
                <details>
                    <summary>{"Markdown Help"}</summary>
                    <div class="help-content">
                        <p><strong>{"Basic Syntax:"}</strong></p>
                        <ul>
                            <li><code>{"**bold**"}</code>{" for "}<strong>{"bold"}</strong></li>
                            <li><code>{"*italic*"}</code>{" for "}<em>{"italic"}</em></li>
                            <li><code>{"# Header 1"}</code>{" for headers"}</li>
                            <li><code>{"[text](url)"}</code>{" for links"}</li>
                            <li><code>{"- item"}</code>{" for lists"}</li>
                            <li><code>{"`code`"}</code>{" for inline code"}</li>
                        </ul>
                    </div>
                </details>
            </div>
        </div>
    }
}