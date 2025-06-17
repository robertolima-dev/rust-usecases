pub mod app_state;
mod settings;

pub use settings::Settings;

use std::sync::OnceLock;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn get_settings() -> &'static Settings {
    SETTINGS.get_or_init(|| Settings::load().expect("Falha ao carregar configurações"))
}

pub fn init_settings() -> Result<(), String> {
    let settings = Settings::load()?;
    SETTINGS
        .set(settings)
        .map_err(|_| "Configurações já inicializadas".to_string())
}
