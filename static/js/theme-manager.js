// ThemeManager: Handles theme and logo changes globally
window.ThemeManager = (function() {
    // Apply a theme by name (loads CSS file and updates variables)
    function applyTheme(themeName) {
        // Remove any existing theme CSS
        var oldLink = document.getElementById('theme-css');
        if (oldLink) oldLink.parentNode.removeChild(oldLink);
        // Add new theme CSS
        var link = document.createElement('link');
        link.rel = 'stylesheet';
        link.id = 'theme-css';
        link.href = '/static/css/themes/' + themeName + '.css';
        document.head.appendChild(link);
        // Persist selection
        localStorage.setItem('selectedTheme', themeName);
        // Dispatch event
        window.dispatchEvent(new CustomEvent('themeChanged', { detail: { theme: themeName } }));
    }

    // Set custom logo (image or text)
    function setCustomLogo(imageUrl, textFallback, font, size) {
        var logoEl = document.getElementById('appLogo');
        if (!logoEl) return;
        if (imageUrl) {
            logoEl.innerHTML = '<img src="' + imageUrl + '" alt="Logo" style="max-height:' + (size || 32) + 'px">';
        } else {
            logoEl.textContent = textFallback || 'Admin App';
            logoEl.style.fontFamily = font || 'Niconne';
            logoEl.style.fontSize = (size || 26) + 'px';
        }
    }

    // On page load, apply persisted theme
    document.addEventListener('DOMContentLoaded', function() {
        var theme = localStorage.getItem('selectedTheme') || 'default';
        applyTheme(theme);
    });

    return {
        applyTheme,
        setCustomLogo
    };
})();

