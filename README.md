# My Rust CMS

A full-stack Content Management System built with the revolutionary **RAYDT Stack** (Rust • Axum • Yew • Diesel • Tower), featuring a Yew WebAssembly frontend and high-performance Axum backend.

[![Built with RAYDT Stack](https://img.shields.io/badge/Built%20with-RAYDT%20Stack-orange.svg)](./RAYDT-STACK.md)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/Frontend-WebAssembly-blue.svg)](https://webassembly.org/)
[![PostgreSQL](https://img.shields.io/badge/Database-PostgreSQL-blue.svg)](https://www.postgresql.org/)

## 🌟 Overview

**My Rust CMS** is a modern, high-performance content management system that leverages the full power of Rust across the entire stack. Built with the groundbreaking **[RAYDT Stack](./RAYDT-STACK.md)**, it delivers enterprise-grade performance, memory safety, and type safety from database to UI.

### 🚀 Key Highlights

- **Memory Safe**: Zero memory-related vulnerabilities thanks to Rust's ownership system
- **Blazing Fast**: Native performance with WebAssembly frontend and optimized Axum backend
- **Type Safe**: Full-stack type safety with compile-time error checking
- **Production Ready**: Enterprise-grade security, authentication, and session management
- **Developer Friendly**: Single language (Rust) across the entire application stack

## ✨ Features

### 🎨 Frontend (Yew WebAssembly)

- **Admin Dashboard**: Comprehensive overview with real-time statistics
- **Visual Page Builder**: Drag-and-drop component-based page creation
- **Post Management**: Rich markdown editor with live preview
- **Media Library**: Intuitive file upload and management interface
- **User Management**: Role-based access control with admin interface
- **Comment Moderation**: Advanced filtering and bulk operations
- **Navigation Manager**: Dynamic menu and navigation configuration
- **Analytics Dashboard**: User engagement and content performance metrics
- **Responsive Design**: Mobile-first, modern UI that works everywhere
- **Public Website**: Fast, SEO-friendly public pages

### ⚡ Backend (Axum + Diesel)

- **RESTful API**: Complete CRUD operations for all resources
- **Authentication System**: Secure session-based auth with bcrypt password hashing
- **Role-Based Access Control**: Admin/user roles with granular permissions
- **Session Management**: Advanced session handling with automatic cleanup
- **File Upload System**: Secure media handling with type validation
- **Database Migrations**: Version-controlled schema with Diesel ORM
- **Rate Limiting**: Built-in protection against abuse
- **CORS Configuration**: Secure cross-origin resource sharing
- **Health Monitoring**: System health checks and performance metrics
- **Background Tasks**: Automated session cleanup and maintenance

### 🗃️ Database Features

- **PostgreSQL Integration**: Production-ready database with ACID compliance
- **Advanced Schema**: 12+ tables supporting complex content relationships
- **Page Builder System**: Flexible component-based page construction
- **Media Management**: Comprehensive file storage and organization
- **User Sessions**: Secure session tracking with expiration
- **Content Categorization**: Hierarchical content organization
- **Navigation System**: Dynamic menu and routing management

## 🏗️ Architecture

### Technology Stack

- **🦀 Backend**: Rust + Axum + Diesel + Tower + PostgreSQL
- **🎨 Frontend**: Rust + Yew + WebAssembly + CSS3
- **🔧 Build Tools**: Cargo workspace + Trunk + wasm-bindgen
- **🐳 Deployment**: Docker + Docker Compose
- **🧪 Testing**: Comprehensive test suite with WASM testing

### Project Structure

```text
my_rust_cms/
├── 📁 backend/                 # Axum API server
│   ├── src/
│   │   ├── controllers/        # Request handlers
│   │   ├── models/            # Database models
│   │   ├── middleware/        # Auth, validation, error handling
│   │   ├── services/          # Business logic
│   │   └── schema.rs          # Database schema
│   └── Cargo.toml
├── 📁 frontend/               # Yew WebAssembly app
│   ├── src/
│   │   ├── components/        # Reusable UI components
│   │   ├── pages/            # Application pages
│   │   ├── services/         # API communication
│   │   └── styles/           # CSS stylesheets
│   └── Cargo.toml
├── 📁 migrations/            # Database migrations
├── 📁 static/               # Static assets
├── 🐳 docker-compose.yml    # Development environment
├── 🐳 Dockerfile           # Production deployment
└── 📚 RAYDT-STACK.md       # Architecture documentation
```

## 🚀 Quick Start

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable): [Install from rustup.rs](https://rustup.rs/)
- **PostgreSQL** (13+): [Install PostgreSQL](https://www.postgresql.org/download/)
- **Trunk** (WebAssembly build tool): `cargo install trunk`
- **Diesel CLI**: `cargo install diesel_cli --features postgres`
- **Docker & Docker Compose** (optional): [Install Docker](https://docs.docker.com/get-docker/)

### Environment Setup

1. **Clone the repository**:

   ```bash
   git clone https://github.com/yourusername/my_rust_cms.git
   cd my_rust_cms
   ```

2. **Set up environment variables**:

   ```bash
   cp .env.example .env  # Create .env file
   ```

      Configure your `.env` file:

   ```env
   # Database Configuration
   DATABASE_URL=postgresql://username:password@localhost:5432/my_rust_cms
   
   # Server Configuration  
   BACKEND_HOST=127.0.0.1
   BACKEND_PORT=8081
   
   # Environment
   RUST_ENV=development
   RUST_LOG=info
   
   # Security (Change in production!)
   JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
   SESSION_SECRET=your-super-secret-session-key-change-this-in-production
   
   # File Upload
   MAX_FILE_SIZE=10485760
      UPLOAD_DIR=./uploads
   ```

3. **Set up the database**:

   ```bash
   # Create database
   createdb my_rust_cms
   
   # Run migrations
   cd backend
   diesel setup
      diesel migration run
   ```

### 🚀 Running the Application

#### Option 1: Development Mode

1. **Start the Backend**:

   ```bash
   cd backend
      cargo run
   ```

   Backend will be available at `http://localhost:8081`

2. **Start the Frontend** (in a new terminal):

   ```bash
   cd frontend
      trunk serve
   ```

   Frontend will be available at `http://localhost:3000`

#### Option 2: Docker Compose (Recommended)

```bash
# Start entire stack with PostgreSQL
docker-compose up -d

# View logs
docker-compose logs -f
```

Access the application at `http://localhost:8080`

### 🔐 Default Credentials

The system automatically creates a default admin user:

- **Username**: `admin`
- **Password**: `admin`

> ⚠️ **Security**: Change the default credentials immediately in production!

## 📚 API Documentation

### Authentication

The API uses session-based authentication. Include the session token in requests requiring authentication.

#### Public Endpoints

```http
POST /api/auth/login          # User login
GET  /api/posts               # List all posts  
GET  /api/posts/:id           # Get specific post
GET  /api/pages               # List all pages
GET  /api/pages/:id           # Get specific page
GET  /api/pages/slug/:slug    # Get page by slug
GET  /api/categories          # List categories
GET  /api/navigation          # Get navigation items
GET  /health                  # Health check
GET  /api/test               # Test endpoint
```

#### Authenticated Endpoints (Requires Login)

```http
POST /api/auth/logout                    # User logout
GET  /api/auth/me                       # Get current user
GET  /api/auth/sessions                 # Get user sessions
POST /api/auth/sessions/logout-all      # Logout all sessions
```

#### Admin-Only Endpoints (Requires Admin Role)

```http
# User Management
GET    /api/users              # List all users
POST   /api/users              # Create new user
PUT    /api/users/:id          # Update user
DELETE /api/users/:id          # Delete user

# Content Management
POST   /api/posts              # Create post
PUT    /api/posts/:id          # Update post  
DELETE /api/posts/:id          # Delete post

GET    /api/comments           # List comments
POST   /api/comments           # Create comment
PUT    /api/comments/:id       # Update comment
DELETE /api/comments/:id       # Delete comment

# Media Management
GET    /api/media              # List media files
POST   /api/media/upload       # Upload media file
DELETE /api/media/:id          # Delete media file

# Page Management
POST   /api/pages              # Create page
PUT    /api/pages/:id          # Update page
DELETE /api/pages/:id          # Delete page

# Navigation Management
POST   /api/navigation              # Create navigation item
PUT    /api/navigation/:id          # Update navigation item  
DELETE /api/navigation/:id          # Delete navigation item

# System Administration
GET    /api/stats                          # System statistics
GET    /api/sessions                       # All sessions
GET    /api/settings                       # System settings
GET    /api/templates                      # Page templates
GET    /api/components                     # Page components
GET    /api/admin/sessions                 # Session statistics
POST   /api/admin/sessions/cleanup         # Manual cleanup
GET    /api/admin/users/:id/sessions       # User sessions
POST   /api/admin/users/:id/force-logout   # Force logout user
```

### Request/Response Examples

#### Login

```bash
curl -X POST http://localhost:8081/api/auth/login \
  -H "Content-Type: application/json" \
    -d '{"username": "admin", "password": "admin"}'
```

#### Create Post (Admin)

```bash
curl -X POST http://localhost:8081/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_SESSION_TOKEN" \
    -d '{"title": "New Post", "content": "Post content", "category_id": 1}'
```

#### Upload Media (Admin)

```bash
curl -X POST http://localhost:8081/api/media/upload \
  -H "Authorization: Bearer YOUR_SESSION_TOKEN" \
    -F "file=@image.jpg"
```

## 🚀 Deployment

### Production Deployment

#### Using Docker (Recommended)

1. **Build the production image**:

   ```bash
      docker build -t my-rust-cms .
   ```

2. **Run with Docker Compose**:

   ```bash
   # Update docker-compose.yml for production
      docker-compose -f docker-compose.prod.yml up -d
   ```

#### Environment Configuration

Create a production `.env` file:

```env
# Production Database
DATABASE_URL=postgresql://prod_user:secure_password@db_host:5432/my_rust_cms_prod

# Production Server
BACKEND_HOST=0.0.0.0
BACKEND_PORT=8080
RUST_ENV=production
RUST_LOG=warn

# Security (Generate secure keys!)
JWT_SECRET=your-production-jwt-secret-at-least-32-characters-long
SESSION_SECRET=your-production-session-secret-at-least-32-characters-long

# File Upload (Adjust for your needs)
MAX_FILE_SIZE=52428800  # 50MB
UPLOAD_DIR=/app/uploads
```

#### Security Checklist

- [ ] Change default admin credentials
- [ ] Generate secure JWT and session secrets
- [ ] Configure proper CORS origins
- [ ] Set up HTTPS/TLS termination
- [ ] Configure firewall rules
- [ ] Set up regular database backups
- [ ] Enable monitoring and logging
- [ ] Review file upload restrictions

### Cloud Deployment

#### AWS Deployment

```bash
# Example using AWS ECS/Fargate
aws ecs create-cluster --cluster-name my-rust-cms
# Configure task definition and service
```

#### Docker Hub

```bash
# Build and push to Docker Hub
docker tag my-rust-cms:latest yourusername/my-rust-cms:latest
docker push yourusername/my-rust-cms:latest
```

## 🧪 Development

### Development Workflow

1. **Make changes to code**
2. **Run tests**:

   ```bash
   # Backend tests
   cd backend && cargo test
   
   # Frontend tests (WASM)
   cd frontend &&    wasm-pack test --headless --firefox
   ```

3. **Check code quality**:

   ```bash
   # Format code
   cargo fmt
   
   # Lint code  
   cargo clippy
   
   # Check for issues
      cargo audit
   ```

### Testing

The project includes comprehensive testing:

- **Unit Tests**: Core business logic testing
- **Integration Tests**: API endpoint testing  
- **WASM Tests**: Frontend component testing
- **Network Tests**: API communication testing

Run all tests:

```bash
# Backend tests
cd backend && cargo test

# Frontend WASM tests
cd frontend && wasm-pack test --headless --firefox

# Integration tests
cd src/tests && cargo test
```

### Database Management

#### Migrations

```bash
# Create new migration
diesel migration generate migration_name

# Apply migrations
diesel migration run

# Revert migrations
diesel migration revert
```

#### Reset Database

```bash
# Drop and recreate (development only!)
diesel database reset
```

### Performance Monitoring

#### Backend Metrics

- Request/response times
- Database query performance
- Memory usage monitoring
- Session management statistics

#### Frontend Metrics

- WASM bundle size optimization
- Page load performance
- Component render times
- Network request efficiency

## 🎯 Features Roadmap

### Current Status ✅

- ✅ **Complete CRUD Operations**: All entities (users, posts, pages, media, comments)
- ✅ **Authentication & Authorization**: Session-based auth with RBAC
- ✅ **Admin Dashboard**: Comprehensive management interface
- ✅ **Visual Page Builder**: Component-based page construction
- ✅ **Media Management**: Secure file upload and organization
- ✅ **Public Website**: Fast, SEO-friendly frontend
- ✅ **Database Migrations**: Version-controlled schema management
- ✅ **Docker Support**: Containerized deployment
- ✅ **Session Management**: Advanced session handling with cleanup
- ✅ **Testing Framework**: Comprehensive test coverage

### Planned Features 🚧

- 🚧 **Email Integration**: Notification system
- 🚧 **Search Functionality**: Full-text search across content
- 🚧 **Analytics Dashboard**: Advanced metrics and insights
- 🚧 **Theme System**: Customizable UI themes
- 🚧 **Plugin Architecture**: Extensible functionality
- 🚧 **API Rate Limiting**: Advanced rate limiting per user/endpoint
- 🚧 **Content Versioning**: Track changes to posts and pages
- 🚧 **Multi-language Support**: Internationalization (i18n)
- 🚧 **Advanced SEO**: Meta tags, sitemaps, structured data
- 🚧 **Backup System**: Automated database and file backups

### Future Enhancements 🔮

- 🔮 **Real-time Collaboration**: Multi-user editing capabilities
- 🔮 **GraphQL API**: Alternative to REST API
- 🔮 **Mobile App**: React Native or Flutter companion
- 🔮 **AI Content Generation**: AI-powered content suggestions
- 🔮 **Advanced Analytics**: Machine learning insights
- 🔮 **Microservices Architecture**: Split into domain services
- 🔮 **Event Sourcing**: Event-driven architecture
- 🔮 **WebRTC Integration**: Real-time communication features

## 🤝 Contributing

We welcome contributions to make this CMS even better!

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Add tests** for new functionality
5. **Ensure all tests pass**: `cargo test && wasm-pack test --headless --firefox`
6. **Format code**: `cargo fmt`
7. **Run lints**: `cargo clippy`
8. **Commit changes**: `git commit -m "Add amazing feature"`
9. **Push to branch**: `git push origin feature/amazing-feature`
10. **Create Pull Request**

### Development Guidelines

- **Follow Rust conventions** and idiomatic patterns
- **Write comprehensive tests** for new features
- **Document public APIs** and complex logic
- **Keep commits focused** and descriptive
- **Update README** for significant changes
- **Maintain backward compatibility** when possible

### Code Style

- Use `cargo fmt` for consistent formatting
- Follow `cargo clippy` recommendations
- Write self-documenting code with clear variable names
- Add comments for complex business logic
- Keep functions focused and single-purpose

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **🦀 Rust Community** for the amazing ecosystem
- **⚡ Axum Team** for the excellent web framework
- **🎨 Yew Contributors** for WebAssembly frontend capabilities
- **🗃️ Diesel Maintainers** for type-safe database interactions
- **🔒 Tower Ecosystem** for composable middleware
- **🌟 RAYDT Stack Pioneers** for pushing the boundaries of web development

## 📞 Support & Community

- **Issues**: [GitHub Issues](https://github.com/yourusername/my_rust_cms/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/my_rust_cms/discussions)
- **Documentation**: [Full Documentation](./docs/)
- **RAYDT Stack**: [Learn More](./RAYDT-STACK.md)

---

**🚀 Built with the revolutionary [RAYDT Stack](./RAYDT-STACK.md)**

*Join in*
