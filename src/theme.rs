// Simple theme configuration for the admin app
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub name: String,
    pub display_name: String,
    pub css_file: String,
    pub logo_text: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            display_name: "Default Theme".to_string(),
            css_file: "/css/themes/default.css".to_string(),
            logo_text: "Admin App".to_string(),
        }
    }
}

pub fn get_available_themes() -> Vec<ThemeConfig> {
    vec![
        ThemeConfig::default(),
        ThemeConfig {
            name: "blue".to_string(),
            display_name: "Blue Corporate".to_string(),
            css_file: "/css/themes/blue.css".to_string(),
            logo_text: "Blue Corp".to_string(),
        },
        ThemeConfig {
            name: "dark".to_string(),
            display_name: "Dark Mode".to_string(),
            css_file: "/css/themes/dark.css".to_string(),
            logo_text: "Dark Admin".to_string(),
        },
        ThemeConfig {
            name: "green".to_string(),
            display_name: "Green Nature".to_string(),
            css_file: "/css/themes/green.css".to_string(),
            logo_text: "Green Nature".to_string(),
        },
    ]
}
