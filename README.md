# 🦀 My Rust CMS

A modern, secure, and scalable **Content Management System** built entirely in Rust, featuring a **Yew frontend** and **Axum backend** with comprehensive security hardening and modular architecture.

## ✨ Features

### 🏗️ **Backend Architecture (Axum)**
- **🔐 Security Hardened**: JWT sessions, rate limiting, CORS protection, input validation
- **🗂️ Modular Controllers**: 9 organized controller modules for maintainability
- **🛡️ Authentication Middleware**: Role-based access control with admin/user permissions
- **📊 Session Management**: Automatic cleanup, monitoring, and lifecycle management
- **⚡ Performance Ready**: Structured error handling, input sanitization, validation
- **🔄 Real-time Features**: Background session cleanup, monitoring endpoints

### 🎨 **Frontend (Yew/WebAssembly)**
- **📱 Responsive Design**: Modern, clean interface with mobile support
- **🎯 Admin Dashboard**: Comprehensive statistics and system monitoring
- **📝 Content Management**: Rich post/page editor with live preview
- **👥 User Management**: Role-based user administration
- **💬 Comment Moderation**: Advanced comment filtering and management
- **📁 Media Library**: Drag-drop file uploads with type validation
- **🔧 Page Builder**: Drag-and-drop component system (in development)

### 🛡️ **Security Features**
- **🔒 Authentication**: Secure session-based auth with bcrypt password hashing
- **🚦 Rate Limiting**: IP-based request throttling to prevent abuse
- **🔍 Input Validation**: Comprehensive sanitization and validation on all endpoints
- **⚠️ Error Handling**: Structured error responses with detailed logging
- **🌐 CORS Protection**: Secure cross-origin configuration
- **🕒 Session Security**: Automatic expiration, cleanup, and monitoring

## 🏛️ Architecture

### 📁 Project Structure
```
my_rust_cms/
├── backend/                    # Axum backend server
│   ├── src/
│   │   ├── controllers/        # 🎯 9 focused API controllers
│   │   │   ├── auth.rs        # Authentication & sessions
│   │   │   ├── users.rs       # User management
│   │   │   ├── posts.rs       # Blog post operations
│   │   │   ├── pages.rs       # Static page management
│   │   │   ├── comments.rs    # Comment system
│   │   │   ├── media.rs       # File upload & management
│   │   │   ├── navigation.rs  # Menu management
│   │   │   ├── sessions.rs    # Session monitoring
│   │   │   └── admin.rs       # System administration
│   │   ├── middleware/         # 🛡️ Security & validation layer
│   │   │   ├── auth.rs        # Authentication middleware
│   │   │   ├── errors.rs      # Structured error handling
│   │   │   └── validation.rs  # Input validation
│   │   ├── services/           # 🔧 Business logic services
│   │   │   └── session_manager.rs  # Session lifecycle management
│   │   ├── models/             # 📊 Database models
│   │   └── main.rs            # 🚀 Application entry (254 lines vs 1260+ before!)
│   └── migrations/             # 🗃️ Database schema migrations
├── frontend/                   # Yew frontend application
│   ├── src/
│   │   ├── components/         # 🎨 UI components
│   │   ├── pages/             # 📄 Application pages
│   │   ├── services/          # 🔗 API integration
│   │   └── styles/            # 💅 CSS styling
│   └── static/                # 📁 Static assets
└── static/                    # 🌐 Shared static files
```

### 🔧 Technology Stack
- **Backend**: Axum, Tokio, Tower-HTTP, Diesel ORM, PostgreSQL
- **Frontend**: Yew, WebAssembly, Gloo, CSS3
- **Security**: bcrypt, JWT sessions, rate limiting, CORS
- **Build Tools**: Trunk (frontend), Cargo (workspace)
- **Database**: PostgreSQL with Diesel migrations

## 🚀 Quick Start

### 📋 Prerequisites
```bash
# Install Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install frontend build tool
cargo install trunk

# Install PostgreSQL
# macOS: brew install postgresql
# Ubuntu: sudo apt install postgresql postgresql-contrib
```

### 🔧 Database Setup
```bash
# Start PostgreSQL service
# macOS: brew services start postgresql
# Ubuntu: sudo systemctl start postgresql

# Create database
createdb rust_cms

# Set environment variables
export DATABASE_URL="postgresql://username:password@localhost/rust_cms"
export JWT_SECRET="your-super-secure-secret-key"
```

### 🏃‍♂️ Running the Application

1. **🗃️ Setup Database**:
   ```bash
   cd backend
   diesel migration run
   ```

2. **🖥️ Start Backend Server**:
   ```bash
   cd backend
   cargo run
   ```
   🌐 Backend available at: `http://localhost:8081`

3. **🎨 Start Frontend**:
   ```bash
   cd frontend
   trunk serve
   ```
   🌐 Frontend available at: `http://localhost:3000`

## 🛣️ API Documentation

### 🔐 Authentication Endpoints
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `POST` | `/api/auth/login` | User login with credentials | ❌ |
| `GET` | `/api/auth/me` | Get current user profile | ✅ |
| `POST` | `/api/auth/logout` | Logout current session | ✅ |
| `GET` | `/api/auth/sessions` | List user's active sessions | ✅ |
| `POST` | `/api/auth/sessions/logout-all` | Logout all user sessions | ✅ |

### 👥 User Management (Admin Only)
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/users` | List all users | 🔒 Admin |
| `POST` | `/api/users` | Create new user | 🔒 Admin |
| `PUT` | `/api/users/:id` | Update user | 🔒 Admin |
| `DELETE` | `/api/users/:id` | Delete user | 🔒 Admin |

### 📝 Content Management
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/posts` | List all posts | ❌ |
| `GET` | `/api/posts/:id` | Get specific post | ❌ |
| `POST` | `/api/posts` | Create new post | 🔒 Admin |
| `PUT` | `/api/posts/:id` | Update post | 🔒 Admin |
| `DELETE` | `/api/posts/:id` | Delete post | 🔒 Admin |

### 📄 Page Management
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/pages` | List all pages | ❌ |
| `GET` | `/api/pages/:id` | Get specific page | ❌ |
| `GET` | `/api/pages/slug/:slug` | Get page by slug | ❌ |
| `POST` | `/api/pages` | Create new page | 🔒 Admin |
| `PUT` | `/api/pages/:id` | Update page | 🔒 Admin |
| `DELETE` | `/api/pages/:id` | Delete page | 🔒 Admin |

### 💬 Comment System
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/comments` | List all comments | 🔒 Admin |
| `POST` | `/api/comments` | Create comment | 🔒 Admin |
| `PUT` | `/api/comments/:id` | Update comment | 🔒 Admin |
| `DELETE` | `/api/comments/:id` | Delete comment | 🔒 Admin |

### 📁 Media Management
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/media` | List media files | 🔒 Admin |
| `POST` | `/api/media/upload` | Upload file | 🔒 Admin |
| `DELETE` | `/api/media/:id` | Delete media file | 🔒 Admin |

### 🧭 Navigation Management
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/navigation` | Get navigation items | ❌ |
| `POST` | `/api/navigation` | Create nav item | 🔒 Admin |
| `PUT` | `/api/navigation/:id` | Update nav item | 🔒 Admin |
| `DELETE` | `/api/navigation/:id` | Delete nav item | 🔒 Admin |

### 📊 Administration & Monitoring
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/api/stats` | System statistics | 🔒 Admin |
| `GET` | `/api/admin/sessions` | Session statistics | 🔒 Admin |
| `POST` | `/api/admin/sessions/cleanup` | Manual session cleanup | 🔒 Admin |
| `GET` | `/api/admin/users/:id/sessions` | User's sessions | 🔒 Admin |
| `POST` | `/api/admin/users/:id/force-logout` | Force logout user | 🔒 Admin |
| `GET` | `/api/categories` | List categories | ❌ |
| `GET` | `/api/settings` | System settings | 🔒 Admin |
| `GET` | `/api/templates` | Page templates | 🔒 Admin |
| `GET` | `/api/components` | Builder components | 🔒 Admin |

### 🔧 System Endpoints
| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/health` | Health check | ❌ |
| `GET` | `/api/test` | Backend test endpoint | ❌ |

## 📊 Data Models

### 👤 User Model
```rust
struct User {
    id: i32,
    username: String,
    password: String,      // bcrypt hashed
    email: Option<String>,
    role: String,          // "admin" | "user"
    status: String,        // "active" | "inactive"
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
```

### 📝 Post Model
```rust
struct Post {
    id: i32,
    title: String,
    content: String,
    category_id: Option<i32>,
    user_id: Option<i32>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
```

### 📄 Page Model
```rust
struct Page {
    id: i32,
    title: String,
    content: String,
    user_id: Option<i32>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
```

### 💬 Comment Model
```rust
struct Comment {
    id: i32,
    post_id: Option<i32>,
    user_id: Option<i32>,
    content: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
```

### 📁 Media Model
```rust
struct Media {
    id: i32,
    file_name: String,
    url: String,
    media_type: Option<String>,
    user_id: Option<i32>,
    uploaded_at: Option<NaiveDateTime>,
}
```

### 🧭 Navigation Model
```rust
struct Navigation {
    id: i32,
    title: String,
    url: String,
    order_position: i32,
    is_active: bool,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}
```

### 🔐 Session Model
```rust
struct Session {
    id: i32,
    user_id: Option<i32>,
    session_token: String,
    expires_at: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
}
```

## 🛡️ Security Features

### 🔐 Authentication System
- **Session-based authentication** with secure token generation
- **Password hashing** using bcrypt with configurable cost
- **Role-based access control** (Admin/User permissions)
- **Automatic session expiration** and cleanup
- **Session monitoring** and management tools

### 🚦 Rate Limiting
- **IP-based rate limiting** to prevent abuse
- **Configurable limits** per endpoint
- **Graceful degradation** with proper error messages

### 🔍 Input Validation
- **Comprehensive validation** for all user inputs
- **SQL injection prevention** through parameterized queries
- **XSS protection** via input sanitization
- **File upload validation** with type and size restrictions

### ⚠️ Error Handling
- **Structured error responses** with appropriate HTTP status codes
- **Detailed logging** for debugging and monitoring
- **User-friendly error messages** without exposing internals

## 📈 Performance Features

- **Efficient database queries** with Diesel ORM
- **Connection pooling** for optimal database performance
- **Async/await** throughout for non-blocking operations
- **Modular architecture** for easy scaling and maintenance
- **Background tasks** for session cleanup and maintenance

## 🗺️ Future Roadmap

### 🔥 High Priority (Next 2-4 weeks)
- [ ] **📄 Pagination System**: Add pagination to all list endpoints
- [ ] **🔍 Database Indexing**: Optimize queries with proper indexing
- [ ] **📦 Response Compression**: Reduce bandwidth with gzip compression
- [ ] **🔐 File Validation**: Enhanced media upload security
- [ ] **🧪 Testing Infrastructure**: Unit and integration tests

### 🚀 Medium Priority (1-3 months)
- [ ] **🔍 Search Functionality**: Full-text search for posts and pages
- [ ] **📊 Analytics Dashboard**: User behavior and system metrics
- [ ] **🔄 Real-time Features**: WebSocket support for live updates
- [ ] **📱 API Rate Limiting**: Per-user quotas and throttling
- [ ] **🎨 Theme System**: Customizable UI themes
- [ ] **📧 Email Integration**: Notifications and password reset
- [ ] **🔒 OAuth Integration**: Social login support

### 🌟 Advanced Features (3-6 months)
- [ ] **🤖 Content AI**: AI-powered content suggestions
- [ ] **🌐 Multi-language**: Internationalization support
- [ ] **📈 Advanced Analytics**: Detailed reporting and insights
- [ ] **🔗 API Versioning**: Backward-compatible API evolution
- [ ] **☁️ Cloud Integration**: S3/CDN for media storage
- [ ] **🔄 Backup System**: Automated database backups
- [ ] **🚀 Performance Monitoring**: APM and health dashboards

### 🏗️ Infrastructure Improvements
- [ ] **🐳 Docker Optimization**: Multi-stage builds and smaller images
- [ ] **📝 OpenAPI Documentation**: Auto-generated API docs
- [ ] **🔧 CI/CD Pipeline**: Automated testing and deployment
- [ ] **📊 Monitoring**: Prometheus metrics and alerting
- [ ] **🔒 Security Auditing**: Regular security assessments

### 🎨 Frontend Enhancements
- [ ] **⚡ Performance**: Lazy loading and code splitting
- [ ] **🎯 UX Improvements**: Loading states and confirmations
- [ ] **🎨 Page Builder**: Complete drag-drop functionality
- [ ] **📱 Mobile App**: React Native companion app
- [ ] **♿ Accessibility**: WCAG 2.1 compliance
- [ ] **🎪 Animations**: Smooth transitions and micro-interactions

## 🤝 Contributing

1. **🍴 Fork the repository**
2. **🌿 Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **💾 Commit changes**: `git commit -m 'Add amazing feature'`
4. **📤 Push to branch**: `git push origin feature/amazing-feature`
5. **🔀 Open a Pull Request**

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **🦀 Rust Community** for the amazing ecosystem
- **⚡ Axum** for the excellent web framework
- **🎨 Yew** for bringing React-like development to Rust
- **🗃️ Diesel** for the robust ORM
- **🔒 Tower** for the middleware ecosystem

---

**Built with ❤️ and 🦀 Rust**

*Ready for production use with comprehensive security, scalability, and maintainability features.*