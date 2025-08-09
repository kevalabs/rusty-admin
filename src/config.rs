// Theme configuration loader and client management
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientConfig {
    pub name: String,
    pub theme: String,
    pub logo: ClientLogo,
    pub custom_css: Option<String>,
    pub custom_js: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientLogo {
    pub text: Option<String>,
    pub image_url: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_theme: String,
    pub themes: HashMap<String, crate::theme::ThemeConfig>,
    pub clients: HashMap<String, ClientConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut themes = HashMap::new();
        
        // Load default themes
        for theme in crate::theme::get_available_themes() {
            themes.insert(theme.name.clone(), theme);
        }
        
        let mut clients = HashMap::new();
        
        // Example client configurations
        clients.insert("client1".to_string(), ClientConfig {
            name: "Acme Corporation".to_string(),
            theme: "blue".to_string(),
            logo: ClientLogo {
                text: Some("Acme Corp".to_string()),
                image_url: Some("/images/logos/acme-logo.png".to_string()),
                width: Some("120px".to_string()),
                height: Some("40px".to_string()),
            },
            custom_css: Some("/css/clients/client1-custom.css".to_string()),
            custom_js: None,
        });
        
        clients.insert("client2".to_string(), ClientConfig {
            name: "Green Solutions Inc".to_string(),
            theme: "green".to_string(),
            logo: ClientLogo {
                text: Some("Green Solutions".to_string()),
                image_url: Some("/images/logos/green-logo.png".to_string()),
                width: Some("140px".to_string()),
                height: Some("35px".to_string()),
            },
            custom_css: Some("/css/clients/client2-custom.css".to_string()),
            custom_js: None,
        });
        
        Self {
            default_theme: "default".to_string(),
            themes,
            clients,
        }
    }
}

pub struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }
    
    pub fn get_client_config(&self, client_id: &str) -> Option<&ClientConfig> {
        self.config.clients.get(client_id)
    }
    
    pub fn get_theme_for_client(&self, client_id: &str) -> &crate::theme::ThemeConfig {
        if let Some(client) = self.get_client_config(client_id) {
            self.config.themes.get(&client.theme)
                .unwrap_or_else(|| self.config.themes.get(&self.config.default_theme).unwrap())
        } else {
            self.config.themes.get(&self.config.default_theme).unwrap()
        }
    }
    
    pub fn list_clients(&self) -> Vec<&ClientConfig> {
        self.config.clients.values().collect()
    }
    
    pub fn add_client(&mut self, client_id: String, config: ClientConfig) {
        self.config.clients.insert(client_id, config);
    }
    
    pub fn export_client_config(&self, client_id: &str) -> Option<String> {
        if let Some(client) = self.get_client_config(client_id) {
            serde_json::to_string_pretty(client).ok()
        } else {
            None
        }
    }
    
    pub fn get_client_css_files(&self, client_id: &str) -> Vec<String> {
        let mut css_files = Vec::new();
        
        if let Some(client) = self.get_client_config(client_id) {
            // Add theme CSS
            if let Some(theme) = self.config.themes.get(&client.theme) {
                css_files.push(theme.css_file.clone());
            }
            
            // Add custom CSS if available
            if let Some(custom_css) = &client.custom_css {
                css_files.push(custom_css.clone());
            }
        } else {
            // Default theme
            if let Some(theme) = self.config.themes.get(&self.config.default_theme) {
                css_files.push(theme.css_file.clone());
            }
        }
        
        css_files
    }
}
