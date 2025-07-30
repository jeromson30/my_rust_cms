// @generated automatically by Diesel CLI.

diesel::table! {
    builder_components (id) {
        id -> Int4,
        component_name -> Varchar,
        component_data -> Nullable<Jsonb>,
        template_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    component_events (id) {
        id -> Int4,
        component_id -> Nullable<Int4>,
        event_type -> Varchar,
        event_handler -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    component_styles (id) {
        id -> Int4,
        component_id -> Nullable<Int4>,
        css -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    components (id) {
        id -> Int4,
        name -> Varchar,
        template_id -> Nullable<Int4>,
        component_data -> Jsonb,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    media (id) {
        id -> Int4,
        file_name -> Varchar,
        url -> Varchar,
        media_type -> Nullable<Varchar>,
        uploaded_at -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    page_components (id) {
        id -> Int4,
        page_id -> Nullable<Int4>,
        component_id -> Nullable<Int4>,
        position -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    page_sections (id) {
        id -> Int4,
        page_id -> Nullable<Int4>,
        section_name -> Varchar,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pages (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        category_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        session_token -> Varchar,
        created_at -> Nullable<Timestamp>,
        expires_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        setting_key -> Varchar,
        setting_value -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    templates (id) {
        id -> Int4,
        name -> Varchar,
        layout -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(builder_components -> templates (template_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(component_events -> components (component_id));
diesel::joinable!(component_styles -> components (component_id));
diesel::joinable!(components -> templates (template_id));
diesel::joinable!(media -> users (user_id));
diesel::joinable!(page_components -> components (component_id));
diesel::joinable!(page_components -> pages (page_id));
diesel::joinable!(page_sections -> pages (page_id));
diesel::joinable!(pages -> users (user_id));
diesel::joinable!(posts -> categories (category_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    builder_components,
    categories,
    comments,
    component_events,
    component_styles,
    components,
    media,
    page_components,
    page_sections,
    pages,
    posts,
    sessions,
    settings,
    templates,
    users,
);
