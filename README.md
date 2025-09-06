# ClubHub 

**Connecting Students with Campus Clubs - A Better Way for Campus Connection at Stanford**

ClubHub is a modern web application that bridges the gap between students and campus clubs at Stanford, providing a seamless platform for communication, information sharing, and community building.

This website changes how students and clubs interact on campus by providing:
- **Club Discovery**: Students can browse and search through campus clubs based on interests, commitment levels, and meeting schedules
- **Direct Messaging**: Two-way communication between students and club leaders
- **Information Push**: Clubs can send announcements, event updates, and important information directly to members
- **Interest Matching**: Smart matching system that connects students with clubs based on their interests and preferences
- **Event Management**: Track club events, RSVPs, and attendance
- **Inbox System**: Centralized messaging hub for all club communications

## Technology Stack

### Backend
- **Rust** - High-performance systems programming language
- **Axum** - Modern, ergonomic web framework for Rust
- **SQLite** - Lightweight, serverless database
- **Rusqlite** - SQLite bindings for Rust
- **Tokio** - Asynchronous runtime for Rust
- **Serde** - Serialization/deserialization framework
- **Chrono** - Date and time library
- **Tower HTTP** - HTTP middleware and services

### Frontend
- **React 18** - Modern JavaScript library for building user interfaces
- **React Router DOM** - Declarative routing for React
- **Axios** - HTTP client for API requests
- **CSS3** - Styling and responsive design

### Development Tools
- **Cargo** - Rust package manager and build system
- **npm** - Node.js package manager
- **Create React App** - React development environment

## Prerequisites

Before setting up ClubHub, ensure you have the following installed:

- **Rust** (latest stable version) - [Install Rust](https://rustup.rs/)
- **Node.js** (v16 or higher) - [Install Node.js](https://nodejs.org/)
- **npm** (comes with Node.js)

Clone the Repository
```bash
git clone <repository-url>
cd grubhub
```

Backend Setup
```bash
cd backend
cargo build
cargo run
```

The backend server will start on `http://localhost:3001` and automatically:
- Create a new SQLite database
- Set up the database schema
- Populate with example data

Frontend Setup
```bash
cd frontend
npm install
npm start
```

The frontend will start on `http://localhost:3000` and automatically open in your browser.

Access the Application
- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:3001

## Project Structure

```
grubhub/
├── backend/                 # Rust backend server
│   ├── src/
│   │   ├── handlers/       # API route handlers
│   │   ├── main.rs         # Application entry point
│   │   ├── connection.rs   # Database connection
│   │   └── types.rs        # Data structures
│   ├── sql/
│   │   └── schema.sql      # Database schema
│   └── Cargo.toml          # Rust dependencies
├── frontend/               # React frontend
│   ├── src/
│   │   ├── components/     # React components
│   │   ├── css/           # Stylesheets
│   │   └── App.js         # Main application component
│   └── package.json       # Node.js dependencies
└── README.md
```

## Key Features

### For Students
- Browse and discover campus clubs
- Search clubs by interests, commitment level, or meeting days
- Join clubs and receive updates
- Manage your club memberships and events
- Centralized inbox for all club communications

### For Club Leaders
- Create and manage club profiles
- Send announcements to members
- Organize events and track attendance
- Communicate directly with interested students
- Push important information to members
