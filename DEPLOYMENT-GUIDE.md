# Rust Admin Panel - Deployment Guide

## 🚀 **Project Overview**

This is a production-ready Rust admin panel with a comprehensive theming system, perfect for white-label deployments and client customization.

## 📋 **Features**

### ✅ **Core Features**
- **Modern Admin Interface** - Clean, responsive Bootstrap 5 design
- **User Management** - Complete user CRUD operations with role-based access
- **Form Validation** - Client-side validation with real-time feedback
- **Profile Management** - User profile editing and settings
- **Notifications System** - Priority-based notification management
- **Dashboard Analytics** - Charts and widgets for data visualization

### 🎨 **Theming System**
- **4 Pre-built Themes** - Default, Blue Corporate, Dark Mode, Green Nature
- **CSS Variable System** - Easy color customization
- **Dynamic Theme Switching** - Real-time theme changes
- **Client Branding** - Logo and color customization
- **White-label Ready** - Client-specific configurations

### 🛠 **Technical Stack**
- **Backend**: Rust with Actix Web framework
- **Templates**: Askama templating engine
- **Frontend**: Bootstrap 5, jQuery, Chart.js
- **Styling**: CSS3 with custom properties
- **Data**: JSON configuration files

## 🏃‍♂️ **Quick Start**

### Prerequisites
- Rust (latest stable version)
- Cargo package manager

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd rusty-admin

# Build the project
cargo build

# Run the application
cargo run
```

### Access the Application
- **Login Page**: http://localhost:8080
- **Dashboard**: http://localhost:8080/dashboard (after login)
- **Theme Settings**: http://localhost:8080/theme-settings

### Default Login Credentials
- **Username**: admin
- **Password**: password

## 🎨 **Client Customization**

### Method 1: Theme Settings UI (Recommended)
1. Navigate to `/theme-settings`
2. Select a pre-built theme
3. Customize logo and colors
4. Save configuration

### Method 2: Custom CSS File
1. Create a new CSS file in `static/css/clients/`
2. Define your brand colors:
```css
:root {
    --theme-primary: #your-brand-color;
    --theme-secondary: #your-secondary-color;
    --theme-header-bg: #your-header-color;
    --theme-sidebar-active: #your-active-color;
}
```

### Method 3: Configuration File
1. Edit `static/config/themes.json`
2. Add your client configuration
3. Reference in the application

## 🚀 **Production Deployment**

### Docker Deployment
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/rust-htmlx-starter /usr/local/bin/app
COPY --from=builder /app/static /static
EXPOSE 8080
CMD ["app"]
```

### Environment Variables
```bash
# Set port (default: 8080)
export PORT=8080

# Set client ID for automatic theme loading
export CLIENT_ID=client1

# Set theme directory
export THEME_DIR=/path/to/themes
```

### Nginx Configuration
```nginx
server {
    listen 80;
    server_name your-domain.com;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
    
    location /static/ {
        alias /path/to/static/;
        expires 1y;
    }
}
```

## 🏢 **Multi-Client Deployment**

### Option 1: Environment-Based
Deploy multiple instances with different environment variables:
```bash
# Client 1
CLIENT_ID=client1 PORT=8081 ./app

# Client 2  
CLIENT_ID=client2 PORT=8082 ./app
```

### Option 2: Domain-Based Routing
Use a reverse proxy to route based on domain:
```nginx
# client1.yourdomain.com -> localhost:8081
# client2.yourdomain.com -> localhost:8082
```

## 📁 **File Structure Reference**

```
├── src/                    # Rust source code
│   ├── main.rs            # Main application entry point
│   ├── theme.rs           # Theme management
│   ├── config.rs          # Configuration management
│   └── middlewares/       # Custom middleware
├── static/                # Static assets
│   ├── css/
│   │   ├── themes/        # Pre-built themes
│   │   └── clients/       # Client-specific CSS
│   ├── js/                # JavaScript files
│   ├── images/logos/      # Client logos
│   └── secured/views/     # HTML templates
└── docs/                  # Documentation
```

## 🔧 **Configuration Files**

### `static/config/themes.json`
Central theme configuration file containing all theme definitions and client mappings.

### `static/css/clients/`
Directory for client-specific CSS overrides and customizations.

### `Cargo.toml`
Rust dependencies and project configuration.

## 🎯 **Customization Examples**

### Corporate Blue Theme
```css
:root {
    --theme-primary: #2c5aa0;
    --theme-secondary: #4a90e2;
    --theme-header-bg: #1e3a5f;
    --theme-sidebar-active: #3d7ed8;
}
```

### Dark Mode Theme
```css
:root {
    --theme-primary: #6366f1;
    --theme-secondary: #8b5cf6;
    --theme-header-bg: #1f2937;
    --theme-sidebar-active: #4f46e5;
    --theme-text: #f9fafb;
    --theme-bg: #111827;
}
```

## 📞 **Support**

For questions or support:
- Check the `THEMING.md` file for detailed theming documentation
- Review `IMPLEMENTATION-SUMMARY.md` for implementation details
- Open an issue in the project repository

## 📄 **License**

This project is licensed under the MIT License. See the LICENSE file for details.

---

**Ready for Production** ✅  
This admin panel is production-ready with comprehensive theming capabilities for multi-client deployments.
