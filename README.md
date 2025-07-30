# My Rust CMS

A modern Content Management System built with Rust, featuring a Yew frontend and Axum backend.

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable version)
- Trunk (for frontend building)
- A modern web browser

### Installation

1. **Install Trunk** (if not already installed):
   ```bash
   cargo install trunk
   ```

2. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd my_rust_cms
   ```

## 🏃‍♂️ Running the Application

### Backend

The backend is a separate Rust application that runs on port 8081.

1. **Navigate to the backend directory**:
   ```bash
   cd backend
   ```

2. **Run the backend**:
   ```bash
   cargo run
   ```

   The backend will start on `http://localhost:8081`

3. **Test the backend**:
   ```bash
   curl http://localhost:8081
   # Should return: "My Rust CMS Backend is running!"
   
   curl http://localhost:8081/health
   # Should return: "OK"
   ```

### Frontend

The frontend is a Yew application that runs on port 3000.

1. **Navigate to the project root**:
   ```bash
   cd /path/to/my_rust_cms
   ```

2. **Run the frontend**:
   ```bash
   trunk serve
   ```

   The frontend will start on `http://localhost:3000`

3. **Open in browser**:
   Navigate to `http://localhost:3000` to see the frontend

## 🔧 Development

### Project Structure

```
my_rust_cms/
├── backend/                 # Backend application
│   ├── src/
│   │   └── main.rs         # Backend entry point
│   └── Cargo.toml          # Backend dependencies
├── src/
│   ├── frontend/           # Frontend application
│   │   ├── main.rs         # Frontend entry point
│   │   ├── services/       # API services
│   │   ├── components/     # UI components
│   │   └── styles/         # CSS styles
│   └── lib.rs              # Library code
├── index.html              # Frontend HTML template
├── Trunk.toml              # Trunk configuration
└── Cargo.toml              # Main project dependencies
```

### Backend Features

- **Axum web framework** for HTTP server
- **Tracing** for logging
- **CORS support** for frontend communication
- **Health check endpoint** at `/health`
- **Root endpoint** at `/`

### Frontend Features

- **Yew framework** for WebAssembly frontend
- **Gloo networking** for HTTP requests
- **Modern CSS styling** with gradients and animations
- **Interactive backend testing** with manual test button
- **Real-time status display** of backend connectivity

## 🌐 API Endpoints

### Current Endpoints

- `GET /` - Backend status message
- `GET /health` - Health check endpoint

### Planned Endpoints

- `POST /auth/login` - User authentication
- `GET /api/posts` - Get all posts
- `POST /api/posts` - Create new post
- `PUT /api/posts/{id}` - Update post
- `DELETE /api/posts/{id}` - Delete post

## 🎨 Frontend Features

- **Responsive design** that works on desktop and mobile
- **Real-time backend connectivity testing**
- **Beautiful UI** with modern gradients and animations
- **Error handling** with user-friendly error messages
- **Loading states** with animated spinners

## 🔄 Development Workflow

1. **Start the backend**:
   ```bash
   cd backend && cargo run
   ```

2. **Start the frontend** (in a new terminal):
   ```bash
   trunk serve
   ```

3. **Make changes** to either frontend or backend code
4. **See changes automatically** - Trunk will rebuild the frontend, and cargo will rebuild the backend

## 🐛 Troubleshooting

### Backend Issues

- **Port already in use**: Change the port in `backend/src/main.rs`
- **Compilation errors**: Check that all dependencies are properly installed

### Frontend Issues

- **Build failures**: Make sure Trunk is installed and up to date
- **CORS errors**: Ensure the backend is running and accessible
- **Network errors**: Check that the backend URL in `index.html` matches your backend port

## 📝 Next Steps

- [ ] Add database integration with Diesel ORM
- [ ] Implement user authentication with JWT
- [ ] Add CRUD operations for posts, users, and categories
- [ ] Create a rich text editor for content creation
- [ ] Add file upload functionality for media
- [ ] Implement user roles and permissions

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test both frontend and backend
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.