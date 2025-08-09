# Theming System Documentation

## Overview

This admin application includes a comprehensive theming system that allows you to customize the appearance for different clients. The system supports:

- **Pre-built themes** (Default, Blue Corporate, Dark Mode, Green Nature)
- **Custom CSS variables** for easy color customization
- **Client-specific configurations** for white-label deployments
- **Logo customization** (text or image-based)
- **Real-time theme switching** with browser persistence

## File Structure

```
static/
├── css/
│   ├── themes/
│   │   ├── default.css      # Default theme
│   │   ├── blue.css         # Blue corporate theme
│   │   ├── dark.css         # Dark mode theme
│   │   └── green.css        # Green nature theme
│   └── clients/
│       ├── client1-custom.css  # Client 1 custom styles
│       └── client2-custom.css  # Client 2 custom styles
├── config/
│   └── themes.json          # Theme configuration
└── images/
    └── logos/               # Client logo images
```

## How to Customize for Clients

### 1. Quick Theme Selection

The easiest way to customize for a client is to:

1. Choose one of the pre-built themes
2. Navigate to `/theme-settings` in the admin panel
3. Select the desired theme
4. Customize the logo text
5. Save the configuration

### 2. Creating Custom CSS

For more advanced customization, create a new CSS file in the `clients/` directory:

```css
/* Client-specific custom styles */
:root {
    --theme-primary: #your-brand-color;
    --theme-secondary: #your-secondary-color;
    --theme-header-bg: #your-header-color;
    --theme-sidebar-active: #your-active-color;
    --theme-logo-text: 'Your Client Name';
}

/* Additional custom styles */
.custom-element {
    background-color: var(--theme-primary);
}
```

### 3. Logo Customization

#### Text Logo
```javascript
// Set custom text logo
window.ThemeManager.setCustomLogo(null, 'Your Client Name');
```

#### Image Logo
```javascript
// Set custom image logo
window.ThemeManager.setCustomLogo('/images/logos/client-logo.png');
```

### 4. Theme Configuration

The `themes.json` file contains all theme configurations:

```json
{
  "themes": {
    "custom-client": {
      "name": "custom-client",
      "display_name": "Custom Client Theme",
      "description": "Custom theme for specific client",
      "colors": {
        "primary": "#your-color",
        "header_bg": "#your-header"
      },
      "logo": {
        "text": "Client Name",
        "image_url": "/images/logos/client-logo.png"
      },
      "css_file": "/css/clients/client-custom.css"
    }
  }
}
```

## CSS Variables Reference

### Colors
- `--theme-primary`: Main brand color
- `--theme-secondary`: Secondary color
- `--theme-success`: Success state color
- `--theme-danger`: Error/danger color
- `--theme-warning`: Warning color
- `--theme-info`: Information color
- `--theme-header-bg`: Header background
- `--theme-sidebar-bg`: Sidebar background
- `--theme-sidebar-active`: Active sidebar item

### Typography
- `--theme-font-family`: Main font family
- `--theme-heading-font`: Heading font family
- `--theme-logo-font`: Logo font family
- `--theme-font-size-base`: Base font size

### Layout
- `--theme-sidebar-width`: Sidebar width
- `--theme-header-height`: Header height
- `--theme-border-radius`: Border radius for elements

## JavaScript API

### Theme Manager Functions

```javascript
// Apply a theme
window.ThemeManager.applyTheme('blue');

// Set custom logo
window.ThemeManager.setCustomLogo(imageUrl, textFallback);

// Get current theme
const currentTheme = window.ThemeManager.getCurrentTheme();

// Get available themes
const themes = window.ThemeManager.getAvailableThemes();
```

### Events

```javascript
// Listen for theme changes
window.addEventListener('themeChanged', function(event) {
    console.log('Theme changed to:', event.detail.theme);
    console.log('Theme config:', event.detail.config);
});
```

## Deployment for Different Clients

### Method 1: Environment-based
1. Set environment variables for client configuration
2. Load appropriate theme on application startup
3. Apply client-specific CSS and logo

### Method 2: URL-based
1. Use subdomain or path to identify client
2. Load client configuration based on URL
3. Apply appropriate theme automatically

### Method 3: Database-driven
1. Store client configurations in database
2. Load theme based on user's client association
3. Dynamic theme loading per user session

## Examples

### Corporate Client
```css
:root {
    --theme-primary: #1e3a8a;
    --theme-header-bg: #1e3a8a;
    --theme-logo-text: 'Corporate Solutions';
}
```

### Healthcare Client
```css
:root {
    --theme-primary: #059669;
    --theme-secondary: #10b981;
    --theme-header-bg: #047857;
    --theme-logo-text: 'HealthCare Pro';
}
```

### Technology Startup
```css
:root {
    --theme-primary: #7c3aed;
    --theme-secondary: #a855f7;
    --theme-header-bg: #6d28d9;
    --theme-logo-text: 'TechStart';
}
```

## Browser Compatibility

The theming system uses modern CSS features:
- CSS Custom Properties (CSS Variables)
- `color-mix()` function for hover states
- CSS Grid and Flexbox for layouts

Supported browsers:
- Chrome 49+
- Firefox 31+
- Safari 9.1+
- Edge 16+

## Performance Considerations

- Themes are loaded via separate CSS files to avoid blocking
- Theme changes are cached in localStorage
- CSS variables provide efficient runtime theme switching
- Minimal JavaScript overhead for theme management

## Troubleshooting

### Theme not applying
1. Check if CSS file exists and is accessible
2. Verify CSS variable names are correct
3. Clear browser cache and localStorage
4. Check browser console for errors

### Logo not displaying
1. Verify image URL is accessible
2. Check image file format (PNG, JPG, SVG supported)
3. Ensure proper file permissions
4. Test with text logo as fallback

### Custom styles not working
1. Use `!important` for overriding default styles
2. Check CSS specificity
3. Verify variable names match exactly
4. Use browser dev tools to debug styles
