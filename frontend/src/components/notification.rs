use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, PartialEq)]
pub struct Notification {
    pub id: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: Option<u32>, // in milliseconds
}

#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub notification: Notification,
    pub on_close: Callback<String>,
}

#[function_component(NotificationItem)]
pub fn notification_item(props: &NotificationProps) -> Html {
    let notification_type_class = match props.notification.notification_type {
        NotificationType::Success => "success",
        NotificationType::Error => "error",
        NotificationType::Warning => "warning",
        NotificationType::Info => "info",
    };

    let on_close = {
        let notification_id = props.notification.id.clone();
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(notification_id.clone()))
    };

    html! {
        <div class={classes!("notification", notification_type_class)}>
            <div class="notification-content">
                <span class="notification-message">{&props.notification.message}</span>
                <button class="notification-close" onclick={on_close}>{"×"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NotificationContainerProps {
    pub notifications: Vec<Notification>,
    pub on_close: Callback<String>,
}

#[function_component(NotificationContainer)]
pub fn notification_container(props: &NotificationContainerProps) -> Html {
    html! {
        <div class="notification-container">
            {props.notifications.iter().map(|notification| {
                html! {
                    <NotificationItem
                        notification={notification.clone()}
                        on_close={props.on_close.clone()}
                    />
                }
            }).collect::<Html>()}
        </div>
    }
} 