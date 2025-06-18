use crate::config::settings::Settings;
use once_cell::sync::OnceCell;
use tracing::info;

static SETTINGS: OnceCell<Settings> = OnceCell::new();

pub fn init_settings() -> Result<(), String> {
    if SETTINGS.get().is_some() {
        info!("⚠️ Settings já inicializado. Ignorando segunda chamada.");
        return Ok(());
    }

    let settings = Settings::load()?;
    SETTINGS
        .set(settings)
        .map_err(|_| "Configurações já inicializadas".to_string())
}

pub fn get_settings() -> &'static Settings {
    SETTINGS
        .get()
        .expect("Settings ainda não inicializado. Rode init_settings() antes.")
}
