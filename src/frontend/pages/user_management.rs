use yew::prelude::*;

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    let users = use_state(|| vec![
        User {
            id: 1,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: "Administrator".to_string(),
            status: "Active".to_string(),
            last_login: "2024-01-15".to_string(),
        },
        User {
            id: 2,
            username: "editor".to_string(),
            email: "editor@example.com".to_string(),
            role: "Editor".to_string(),
            status: "Active".to_string(),
            last_login: "2024-01-14".to_string(),
        },
    ]);

    html! {
        <div class="user-management">
            <div class="page-header">
                <h1>{"User Management"}</h1>
                <button class="btn-primary">{"Add New User"}</button>
            </div>

            <div class="users-table">
                <table>
                    <thead>
                        <tr>
                            <th>{"Username"}</th>
                            <th>{"Email"}</th>
                            <th>{"Role"}</th>
                            <th>{"Status"}</th>
                            <th>{"Last Login"}</th>
                            <th>{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {users.iter().map(|user| html! {
                            <tr key={user.id}>
                                <td>{&user.username}</td>
                                <td>{&user.email}</td>
                                <td>
                                    <span class="role-badge">{&user.role}</span>
                                </td>
                                <td>
                                    <span class="status-badge active">{&user.status}</span>
                                </td>
                                <td>{&user.last_login}</td>
                                <td>
                                    <div class="action-buttons">
                                        <button class="btn-small">{"Edit"}</button>
                                        <button class="btn-small btn-danger">{"Delete"}</button>
                                    </div>
                                </td>
                            </tr>
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct User {
    id: u32,
    username: String,
    email: String,
    role: String,
    status: String,
    last_login: String,
}
