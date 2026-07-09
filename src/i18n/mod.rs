// ────────────────────────────────────────────────────────────────────────────────
// Pag — Moduł internacjonalizacji (i18n)
// ────────────────────────────────────────────────────────────────────────────────

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Struktura tłumaczeń
pub struct I18n {
    pub lang: String,
    messages: &'static HashMap<&'static str, &'static str>,
}

impl I18n {
    pub fn new(lang: &str) -> Self {
        let messages = match lang {
            "pl" => &PL_MESSAGES,
            "de" => &DE_MESSAGES,
            "fr" => &FR_MESSAGES,
            "es" => &ES_MESSAGES,
            _ => &EN_MESSAGES,
        };

        I18n {
            lang: lang.to_string(),
            messages,
        }
    }

    pub fn t(&self, key: &str) -> String {
        self.messages
            .get(key)
            .copied()
            .unwrap_or(key)
            .to_string()
    }

    pub fn tf(&self, key: &str, args: &[(&str, &str)]) -> String {
        let template = self.t(key);
        let mut result = template;
        for (k, v) in args {
            result = result.replace(&format!("{{{}}}", k), v);
        }
        result
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Tłumaczenia — Polski (domyślny)
// ────────────────────────────────────────────────────────────────────────────────

static PL_MESSAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        // Ogólne
        ("loading_db", "Wczytywanie bazy danych..."),
        ("saving_db", "Zapisywanie bazy danych..."),
        ("done", "Gotowe!"),
        ("error", "Błąd"),
        ("warning", "Ostrzeżenie"),
        ("info", "Info"),
        ("yes", "tak"),
        ("no", "nie"),
        ("aborted", "Aborted."),
        ("no_changes", "No changes."),

        // Instalacja
        ("installing", "Instalowanie"),
        ("installed", "Zainstalowano"),
        ("install_confirm", "Czy na pewno chcesz zainstalować {packages}?"),
        ("install_resolving", "Rozwiązywanie zależności..."),
        ("install_deps", "Zależności do instalacji:"),
        ("install_downloading", "Pobieranie pakietów..."),
        ("install_extracting", "Rozpakowywanie pakietów..."),
        ("install_configuring", "Konfigurowanie..."),
        ("install_hooks", "Uruchamianie hooków instalacyjnych..."),

        // Usuwanie
        ("removing", "Usuwanie"),
        ("removed", "Usunięto"),
        ("remove_confirm", "Czy na pewno chcesz usunąć {packages}?"),
        ("remove_deps_check", "Sprawdzanie zależności..."),
        ("remove_orphans", "Osierocone zależności:"),

        // Aktualizacja
        ("upgrading", "Aktualizowanie"),
        ("upgraded", "Zaktualizowano"),
        ("upgrade_checking", "Sprawdzanie availablech aktualizacji..."),
        ("upgrade_available", "Available updates:"),
        ("upgrade_sys_checking", "Sprawdzanie nowej wersji systemu..."),
        ("upgrade_sys_new", "Nowa wersja systemu dostępna: {version}"),
        ("upgrade_sys_current", "System jest aktualny."),

        // Synchronizacja
        ("syncing", "Synchronizing repository databases..."),
        ("sync_repo", "Synchronizowanie {repo}..."),
        ("synced", "Bazy zsynchronizowane."),

        // Searching
        ("searching", "Searching..."),
        ("search_results", "Wyniki wyszukiwania dla \"{query}\":"),
        ("search_no_results", "No packages found."),

        // Informacje
        ("info_name", "Name"),
        ("info_version", "Version"),
        ("info_desc", "Description"),
        ("info_arch", "Architektura"),
        ("info_size", "Rozmiar"),
        ("info_license", "Licencja"),
        ("info_deps", "Zależności"),
        ("info_url", "URL"),

        // Budowanie
        ("building", "Budowanie"),
        ("build_start", "Rozpoczynanie budowania {package}..."),
        ("build_configuring", "Konfigurowanie..."),
        ("build_compiling", "Kompilowanie..."),
        ("build_packaging", "Pakowanie..."),
        ("build_signing", "Podpisywanie..."),
        ("build_success", "Pomyślnie zbudowano: {file}"),
        ("build_failed", "Budowanie nie powiodło się."),

        // Migawki
        ("snap_create", "Tworzenie migawki..."),
        ("snap_created", "Migawka utworzona: {id}"),
        ("snap_restore", "Przywracanie migawki {id}..."),
        ("snap_restored", "Migawka przywrócona. Zrestartuj system."),
        ("snap_delete", "Usuwanie migawki {id}..."),
        ("snap_deleted", "Migawka usunięta."),

        // Błędy
        ("err_root", "Ta operacja wymaga uprawnień roota."),
        ("err_not_found", "Package not found: {package}"),
        ("err_sig", "Invalid signature pakietu: {package}"),
        ("err_deps", "Nie można rozwiązać zależności."),
        ("err_locked", "Baza danych jest zablokowana przez inny proces."),
        ("err_network", "Błąd sieci: {msg}"),
        ("err_checksum", "Nieprawidłowa suma kontrolna: {file}"),
    ])
});

// ────────────────────────────────────────────────────────────────────────────────
// Tłumaczenia — Angielski
// ────────────────────────────────────────────────────────────────────────────────

static EN_MESSAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("loading_db", "Loading database..."),
        ("saving_db", "Saving database..."),
        ("done", "Done!"),
        ("error", "Error"),
        ("warning", "Warning"),
        ("info", "Info"),
        ("yes", "yes"),
        ("no", "no"),
        ("aborted", "Aborted."),
        ("no_changes", "No changes."),

        ("installing", "Installing"),
        ("installed", "Installed"),
        ("install_confirm", "Are you sure you want to install {packages}?"),
        ("install_resolving", "Resolving dependencies..."),
        ("install_deps", "Dependencies to install:"),
        ("install_downloading", "Downloading packages..."),
        ("install_extracting", "Extracting packages..."),
        ("install_configuring", "Configuring..."),
        ("install_hooks", "Running install hooks..."),

        ("removing", "Removing"),
        ("removed", "Removed"),
        ("remove_confirm", "Are you sure you want to remove {packages}?"),
        ("remove_deps_check", "Checking dependencies..."),
        ("remove_orphans", "Orphaned dependencies:"),

        ("upgrading", "Upgrading"),
        ("upgraded", "Upgraded"),
        ("upgrade_checking", "Checking for updates..."),
        ("upgrade_available", "Available updates:"),
        ("upgrade_sys_checking", "Checking for new system version..."),
        ("upgrade_sys_new", "New system version available: {version}"),
        ("upgrade_sys_current", "System is up to date."),

        ("syncing", "Synchronizing repository databases..."),
        ("sync_repo", "Synchronizing {repo}..."),
        ("synced", "Databases synchronized."),

        ("searching", "Searching..."),
        ("search_results", "Search results for \"{query}\":"),
        ("search_no_results", "No packages found."),

        ("info_name", "Name"),
        ("info_version", "Version"),
        ("info_desc", "Description"),
        ("info_arch", "Architecture"),
        ("info_size", "Size"),
        ("info_license", "License"),
        ("info_deps", "Dependencies"),
        ("info_url", "URL"),

        ("building", "Building"),
        ("build_start", "Starting build of {package}..."),
        ("build_configuring", "Configuring..."),
        ("build_compiling", "Compiling..."),
        ("build_packaging", "Packaging..."),
        ("build_signing", "Signing..."),
        ("build_success", "Successfully built: {file}"),
        ("build_failed", "Build failed."),

        ("snap_create", "Creating snapshot..."),
        ("snap_created", "Snapshot created: {id}"),
        ("snap_restore", "Restoring snapshot {id}..."),
        ("snap_restored", "Snapshot restored. Please reboot."),
        ("snap_delete", "Deleting snapshot {id}..."),
        ("snap_deleted", "Snapshot deleted."),

        ("err_root", "This operation requires root privileges."),
        ("err_not_found", "Package not found: {package}"),
        ("err_sig", "Invalid package signature: {package}"),
        ("err_deps", "Cannot resolve dependencies."),
        ("err_locked", "Database is locked by another process."),
        ("err_network", "Network error: {msg}"),
        ("err_checksum", "Checksum mismatch: {file}"),
    ])
});

// ────────────────────────────────────────────────────────────────────────────────
// Tłumaczenia — Niemiecki
// ────────────────────────────────────────────────────────────────────────────────

static DE_MESSAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("loading_db", "Lade Datenbank..."),
        ("saving_db", "Speichere Datenbank..."),
        ("done", "Fertig!"),
        ("error", "Fehler"),
        ("warning", "Warnung"),
        ("info", "Info"),
        ("yes", "ja"),
        ("no", "nein"),
        ("aborted", "Abgebrochen."),
        ("no_changes", "Keine Änderungen."),

        ("installing", "Installiere"),
        ("installed", "Installiert"),
        ("install_confirm", "Möchten Sie wirklich {packages} installieren?"),
        ("install_resolving", "Löse Abhängigkeiten auf..."),
        ("install_deps", "Zu installierende Abhängigkeiten:"),
        ("install_downloading", "Lade Pakete herunter..."),
        ("install_extracting", "Entpacke Pakete..."),
        ("install_configuring", "Konfiguriere..."),
        ("install_hooks", "Führe Installations-Hooks aus..."),

        ("removing", "Entferne"),
        ("removed", "Entfernt"),
        ("remove_confirm", "Möchten Sie wirklich {packages} entfernen?"),

        ("upgrading", "Aktualisiere"),
        ("upgraded", "Aktualisiert"),
        ("upgrade_checking", "Suche nach Aktualisierungen..."),
        ("upgrade_available", "Verfügbare Aktualisierungen:"),
        ("upgrade_sys_checking", "Suche nach neuer Systemversion..."),
        ("upgrade_sys_new", "Neue Systemversion verfügbar: {version}"),
        ("upgrade_sys_current", "System ist aktuell."),

        ("syncing", "Synchronisiere Repository-Datenbanken..."),
        ("sync_repo", "Synchronisiere {repo}..."),
        ("synced", "Datenbanken synchronisiert."),

        ("searching", "Suche..."),
        ("search_results", "Suchergebnisse für \"{query}\":"),
        ("search_no_results", "Keine Pakete gefunden."),

        ("build_start", "Starte Build von {package}..."),
        ("build_success", "Erfolgreich gebaut: {file}"),
        ("build_failed", "Build fehlgeschlagen."),

        ("snap_create", "Erstelle Snapshot..."),
        ("snap_created", "Snapshot erstellt: {id}"),
    ])
});

// ────────────────────────────────────────────────────────────────────────────────
// Tłumaczenia — Francuski
// ────────────────────────────────────────────────────────────────────────────────

static FR_MESSAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("loading_db", "Chargement de la base de données..."),
        ("saving_db", "Sauvegarde de la base de données..."),
        ("done", "Terminé !"),
        ("error", "Erreur"),
        ("warning", "Attention"),
        ("info", "Info"),
        ("yes", "oui"),
        ("no", "non"),
        ("aborted", "Annulé."),
        ("no_changes", "Aucun changement."),

        ("installing", "Installation de"),
        ("installed", "Installé"),
        ("install_confirm", "Voulez-vous vraiment installer {packages} ?"),
        ("install_resolving", "Résolution des dépendances..."),
        ("install_deps", "Dépendances à installer :"),
        ("install_downloading", "Téléchargement des paquets..."),
        ("install_extracting", "Extraction des paquets..."),

        ("removing", "Suppression de"),
        ("removed", "Supprimé"),

        ("upgrading", "Mise à jour de"),
        ("upgraded", "Mis à jour"),
        ("upgrade_checking", "Recherche de mises à jour..."),
        ("upgrade_available", "Mises à jour disponibles :"),
        ("upgrade_sys_checking", "Recherche d'une nouvelle version du système..."),
        ("upgrade_sys_new", "Nouvelle version du système disponible : {version}"),
        ("upgrade_sys_current", "Le système est à jour."),

        ("syncing", "Synchronisation des dépôts..."),
        ("sync_repo", "Synchronisation de {repo}..."),
        ("synced", "Dépôts synchronisés."),

        ("searching", "Recherche..."),
        ("search_results", "Résultats pour \"{query}\" :"),
        ("search_no_results", "Aucun paquet trouvé."),

        ("build_start", "Début de la construction de {package}..."),
        ("build_success", "Construction réussie : {file}"),
        ("build_failed", "Échec de la construction."),
    ])
});

// ────────────────────────────────────────────────────────────────────────────────
// Tłumaczenia — Hiszpański
// ────────────────────────────────────────────────────────────────────────────────

static ES_MESSAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("loading_db", "Cargando base de datos..."),
        ("saving_db", "Guardando base de datos..."),
        ("done", "¡Hecho!"),
        ("error", "Error"),
        ("warning", "Advertencia"),
        ("info", "Info"),
        ("yes", "sí"),
        ("no", "no"),
        ("aborted", "Abortado."),
        ("no_changes", "Sin cambios."),

        ("installing", "Instalando"),
        ("installed", "Instalado"),
        ("install_confirm", "¿Está seguro de instalar {packages}?"),
        ("install_resolving", "Resolviendo dependencias..."),
        ("install_deps", "Dependencias a instalar:"),
        ("install_downloading", "Descargando paquetes..."),
        ("install_extracting", "Extrayendo paquetes..."),

        ("removing", "Eliminando"),
        ("removed", "Eliminado"),

        ("upgrading", "Actualizando"),
        ("upgraded", "Actualizado"),
        ("upgrade_checking", "Buscando actualizaciones..."),
        ("upgrade_available", "Actualizaciones disponibles:"),
        ("upgrade_sys_checking", "Buscando nueva versión del sistema..."),
        ("upgrade_sys_new", "Nueva versión del sistema disponible: {version}"),
        ("upgrade_sys_current", "El sistema está actualizado."),

        ("syncing", "Sincronizando repositorios..."),
        ("sync_repo", "Sincronizando {repo}..."),
        ("synced", "Repositorios sincronizados."),

        ("searching", "Buscando..."),
        ("search_results", "Resultados para \"{query}\":"),
        ("search_no_results", "No se encontraron paquetes."),

        ("build_start", "Iniciando construcción de {package}..."),
        ("build_success", "Construido exitosamente: {file}"),
        ("build_failed", "Construcción fallida."),
    ])
});
