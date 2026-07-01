// Moduł internacjonalizacji - uproszczona wersja

use std::collections::HashMap;

pub struct I18n {
    pub lang: String,
    messages: HashMap<String, String>,
}

impl I18n {
    pub fn new(lang: &str) -> Self {
        let messages = match lang {
            "pl" => pl_messages(),
            "de" => de_messages(),
            "fr" => fr_messages(),
            "es" => es_messages(),
            "it" => it_messages(),
            "ru" => ru_messages(),
            "cs" => cs_messages(),
            "ja" => ja_messages(),
            "zh" => zh_messages(),
            _ => en_messages(),
        };

        Self {
            lang: lang.to_string(),
            messages,
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(|s| s.as_str())
    }
}

pub fn init(lang: &str) -> I18n {
    I18n::new(lang)
}

fn en_messages() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("pag.welcome".into(), "PaganLinux Package Manager v0.1.0".into());
    m.insert("pag.installing".into(), "Installing {pkg}...".into());
    m.insert("pag.installed".into(), "Successfully installed {pkg}".into());
    m.insert("pag.removing".into(), "Removing {pkg}...".into());
    m.insert("pag.error".into(), "Error: {msg}".into());
    m
}

fn pl_messages() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("pag.welcome".into(), "PaganLinux Package Manager v0.1.0".into());
    m.insert("pag.installing".into(), "Instalowanie {pkg}...".into());
    m.insert("pag.installed".into(), "Pomyślnie zainstalowano {pkg}".into());
    m.insert("pag.removing".into(), "Usuwanie {pkg}...".into());
    m.insert("pag.error".into(), "Błąd: {msg}".into());
    m
}

fn de_messages() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("pag.welcome".into(), "PaganLinux Paketmanager v0.1.0".into());
    m.insert("pag.installing".into(), "Installiere {pkg}...".into());
    m.insert("pag.installed".into(), "{pkg} erfolgreich installiert".into());
    m.insert("pag.removing".into(), "Entferne {pkg}...".into());
    m.insert("pag.error".into(), "Fehler: {msg}".into());
    m
}

fn fr_messages() -> HashMap<String, String> { en_messages() }
fn es_messages() -> HashMap<String, String> { en_messages() }
fn it_messages() -> HashMap<String, String> { en_messages() }
fn ru_messages() -> HashMap<String, String> { en_messages() }
fn cs_messages() -> HashMap<String, String> { en_messages() }
fn ja_messages() -> HashMap<String, String> { en_messages() }
fn zh_messages() -> HashMap<String, String> { en_messages() }
