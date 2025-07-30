# My Rust CMS

A full-stack Content Management System built with Rust, featuring a Yew frontend and Axum backend.

## Features Implemented

### Backend API (Axum)
- **Posts Management**: Full CRUD operations for blog posts
- **User Management**: Full CRUD operations for user accounts
- **Comment Moderation**: Full CRUD operations for comments
- **Media Library**: Create and delete media items
- **Statistics**: System statistics endpoint
- **CORS Support**: Cross-origin resource sharing enabled

### Frontend (Yew/WASM)
- **Dashboard**: Overview with statistics and recent posts
- **Posts Management**: Create, read, update, delete posts with forms
- **User Management**: Create, read, update, delete users with role management
- **Comment Moderation**: Manage comments with filtering and status updates
- **Media Library**: Upload and manage media files
- **Responsive UI**: Modern, clean interface with navigation

## Architecture

### Workspace Structure
```
my_rust_cms/
├── backend/          # Axum backend server
├── frontend/         # Yew frontend application
├── static/           # Static assets
└── Cargo.toml        # Workspace configuration
```

### Technology Stack
- **Backend**: Axum, Tokio, Tower-HTTP, Serde
- **Frontend**: Yew, WebAssembly, Gloo
- **Build Tool**: Trunk (for frontend)
- **Package Manager**: Cargo

## Getting Started

### Prerequisites
- Rust (latest stable)
- Trunk (`cargo install trunk`)

### Running the Application

1. **Start the Backend**:
   ```bash
   cd backend
   cargo run
   ```
   The backend will be available at `http://localhost:8081`

2. **Start the Frontend**:
   ```bash
   cd frontend
   trunk serve
   ```
   The frontend will be available at `http://localhost:3000`

### API Endpoints

#### Posts
- `GET /api/posts` - List all posts
- `POST /api/posts` - Create a new post
- `PUT /api/posts/:id` - Update a post
- `DELETE /api/posts/:id` - Delete a post

#### Users
- `GET /api/users` - List all users
- `POST /api/users` - Create a new user
- `PUT /api/users/:id` - Update a user
- `DELETE /api/users/:id` - Delete a user

#### Comments
- `GET /api/comments` - List all comments
- `POST /api/comments` - Create a new comment
- `PUT /api/comments/:id` - Update a comment
- `DELETE /api/comments/:id` - Delete a comment

#### Media
- `GET /api/media` - List all media items
- `POST /api/media` - Create a new media item
- `DELETE /api/media/:id` - Delete a media item

#### System
- `GET /api/stats` - Get system statistics
- `GET /health` - Health check endpoint

## Current Status

✅ **Fully Functional CMS with:**
- Complete CRUD operations for all entities
- Real-time data synchronization between frontend and backend
- Modern, responsive user interface
- Error handling and loading states
- Form validation and user feedback

The application is now ready for use with full functionality for managing posts, users, comments, and media files.