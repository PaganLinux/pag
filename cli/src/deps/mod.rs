// Moduł rozwiązywania zależności
//
// Zaawansowany solver zależności z obsługą:
// - Wersjonowanych zależności (>=, <=, =, >, <)
// - Wirtualnych pakietów (provides)
// - Konfliktów
// - Zależności opcjonalnych
// - Rozwiązywania cykli

use std::collections::{HashMap, HashSet, VecDeque};

/// Reprezentuje zależność pakietu
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version_req: Option<VersionReq>,
}

/// Wymaganie wersji
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionReq {
    Any,
    Eq(String),
    Gt(String),
    Gte(String),
    Lt(String),
    Lte(String),
}

/// Wynik rozwiązania zależności
#[derive(Debug)]
pub struct DepsResult {
    /// Pakiety do zainstalowania (w kolejności)
    pub to_install: Vec<String>,
    /// Pakiety do aktualizacji
    pub to_upgrade: Vec<(String, String, String)>, // (name, old_ver, new_ver)
    /// Pakiety do usunięcia (konflikty)
    pub to_remove: Vec<String>,
    /// Niespełnione zależności
    pub missing: Vec<String>,
    /// Konflikty
    pub conflicts: Vec<(String, String)>,
    /// Ostrzeżenia
    pub warnings: Vec<String>,
}

impl Dependency {
    /// Parsuje string zależności, np. "glibc>=2.39", "libcurl=8.0"
    pub fn parse(dep: &str) -> Self {
        let dep = dep.trim();

        // Sprawdź operatory wersji
        for (op, variant_fn) in [
            (">=", VersionReq::Gte as fn(String) -> VersionReq),
            ("<=", VersionReq::Lte),
            (">", VersionReq::Gt),
            ("<", VersionReq::Lt),
            ("=", VersionReq::Eq),
        ] {
            if let Some(pos) = dep.find(op) {
                let name = dep[..pos].to_string();
                let version = dep[pos + op.len()..].to_string();
                return Dependency {
                    name,
                    version_req: Some(variant_fn(version)),
                };
            }
        }

        // Brak wersji
        Dependency {
            name: dep.to_string(),
            version_req: None,
        }
    }

    /// Sprawdza czy wersja spełnia wymaganie
    pub fn matches(&self, version: &str) -> bool {
        match &self.version_req {
            None => true,
            Some(req) => req.matches(version),
        }
    }
}

impl VersionReq {
    pub fn matches(&self, version: &str) -> bool {
        match self {
            VersionReq::Any => true,
            VersionReq::Eq(v) => compare_versions(version, v) == std::cmp::Ordering::Equal,
            VersionReq::Gt(v) => compare_versions(version, v) == std::cmp::Ordering::Greater,
            VersionReq::Gte(v) => compare_versions(version, v) != std::cmp::Ordering::Less,
            VersionReq::Lt(v) => compare_versions(version, v) == std::cmp::Ordering::Less,
            VersionReq::Lte(v) => compare_versions(version, v) != std::cmp::Ordering::Greater,
        }
    }
}

/// Porównuje dwie wersje (obsługuje format semver i epoch)
fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let a_clean = a.trim_start_matches('v').trim();
    let b_clean = b.trim_start_matches('v').trim();

    // Podziel na części numeryczne i tekstowe
    let a_parts: Vec<&str> = a_clean.split(|c: char| !c.is_alphanumeric()).collect();
    let b_parts: Vec<&str> = b_clean.split(|c: char| !c.is_alphanumeric()).collect();

    for i in 0..a_parts.len().max(b_parts.len()) {
        let a_part = a_parts.get(i).copied().unwrap_or("");
        let b_part = b_parts.get(i).copied().unwrap_or("");

        // Spróbuj porównać numerycznie
        if let (Ok(a_num), Ok(b_num)) = (a_part.parse::<u64>(), b_part.parse::<u64>()) {
            match a_num.cmp(&b_num) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }

        // Porównaj leksykograficznie
        match a_part.cmp(b_part) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }

    std::cmp::Ordering::Equal
}

/// Rozwiązuje zależności dla zadanych pakietów
pub struct DepSolver {
    /// Zainstalowane pakiety: nazwa -> wersja
    installed: HashMap<String, String>,
    /// Dostępne pakiety: nazwa -> (wersja, zależności, provides)
    available: HashMap<String, PackageInfo>,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub version: String,
    pub depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub repo: String,
    pub install_size: u64,
}

impl DepSolver {
    pub fn new() -> Self {
        Self {
            installed: HashMap::new(),
            available: HashMap::new(),
        }
    }

    /// Dodaje zainstalowany pakiet do solwera
    pub fn add_installed(&mut self, name: &str, version: &str) {
        self.installed.insert(name.to_string(), version.to_string());
    }

    /// Dodaje dostępny pakiet do solwera
    pub fn add_available(&mut self, name: &str, info: PackageInfo) {
        self.available.insert(name.to_string(), info);
    }

    /// Rozwiązuje zależności
    pub fn solve(&self, packages: &[String], install: bool) -> DepsResult {
        let mut result = DepsResult {
            to_install: Vec::new(),
            to_upgrade: Vec::new(),
            to_remove: Vec::new(),
            missing: Vec::new(),
            conflicts: Vec::new(),
            warnings: Vec::new(),
        };

        let mut visited = HashSet::new();
        let mut queue: VecDeque<String> = packages.iter().cloned().collect();
        let mut to_install_set = HashSet::new();

        while let Some(pkg_name) = queue.pop_front() {
            if visited.contains(&pkg_name) {
                continue;
            }
            visited.insert(pkg_name.clone());

            // Sprawdź provides
            let actual_name = self.resolve_provide(&pkg_name);

            if let Some((name, info)) = actual_name {
                // Sprawdź czy już zainstalowany
                if let Some(installed_ver) = self.installed.get(&name) {
                    // Sprawdź czy wymaga aktualizacji
                    if compare_versions(&info.version, installed_ver) == std::cmp::Ordering::Greater {
                        result.to_upgrade.push((
                            name.clone(),
                            installed_ver.clone(),
                            info.version.clone(),
                        ));
                    }
                } else {
                    // Dodaj do instalacji
                    if install {
                        to_install_set.insert(name.clone());
                    }
                }

                // Sprawdź konflikty
                for conflict in &info.conflicts {
                    let dep = Dependency::parse(conflict);
                    if let Some(conflict_ver) = self.installed.get(&dep.name) {
                        if dep.matches(conflict_ver) {
                            result.conflicts.push((name.clone(), format!("{}={}", dep.name, conflict_ver)));
                        }
                    }
                }

                // Dodaj zależności do kolejki
                for dep_str in &info.depends {
                    let dep = Dependency::parse(dep_str);
                    if !self.is_satisfied(&dep) {
                        queue.push_back(dep.name.clone());
                    }
                }
            } else {
                // Pakiet niedostępny
                result.missing.push(pkg_name.clone());
            }
        }

        // Topologiczne sortowanie (zależności przed pakietami które ich potrzebują)
        result.to_install = self.topo_sort(&to_install_set);

        result
    }

    /// Znajduje pakiet (w tym przez provides)
    fn resolve_provide(&self, name: &str) -> Option<(String, PackageInfo)> {
        // Szukaj dokładnej nazwy
        if let Some(info) = self.available.get(name) {
            return Some((name.to_string(), info.clone()));
        }

        // Szukaj przez provides
        for (pkg_name, info) in &self.available {
            if info.provides.iter().any(|p| p == name || p.starts_with(&format!("{}=", name))) {
                return Some((pkg_name.clone(), info.clone()));
            }
        }

        // Sprawdź czy już zainstalowany (zależność spełniona)
        if self.installed.contains_key(name) {
            return None; // Już spełnione
        }

        None
    }

    /// Sprawdza czy zależność jest spełniona
    fn is_satisfied(&self, dep: &Dependency) -> bool {
        // Sprawdź zainstalowane
        for (name, version) in &self.installed {
            if dep.name == *name {
                return dep.matches(version);
            }
        }

        // Sprawdź provides w zainstalowanych
        // (uproszczona wersja — pełna wymaga bazy danych)
        false
    }

    /// Sortowanie topologiczne
    fn topo_sort(&self, packages: &HashSet<String>) -> Vec<String> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();

        for pkg in packages {
            if !visited.contains(pkg) {
                self.topo_visit(pkg, packages, &mut visited, &mut temp_mark, &mut sorted);
            }
        }

        sorted
    }

    fn topo_visit(
        &self,
        pkg: &str,
        all_packages: &HashSet<String>,
        visited: &mut HashSet<String>,
        temp_mark: &mut HashSet<String>,
        sorted: &mut Vec<String>,
    ) {
        if temp_mark.contains(pkg) {
            // Cykl - przerywamy
            return;
        }
        if visited.contains(pkg) {
            return;
        }

        temp_mark.insert(pkg.to_string());

        // Odwiedź zależności
        if let Some(info) = self.available.get(pkg) {
            for dep_str in &info.depends {
                let dep = Dependency::parse(dep_str);
                if all_packages.contains(&dep.name) {
                    self.topo_visit(&dep.name, all_packages, visited, temp_mark, sorted);
                }
            }
        }

        temp_mark.remove(pkg);
        visited.insert(pkg.to_string());
        sorted.push(pkg.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_parse() {
        let dep = Dependency::parse("glibc>=2.39");
        assert_eq!(dep.name, "glibc");
        assert!(dep.matches("2.40"));
        assert!(!dep.matches("2.38"));
    }

    #[test]
    fn test_version_comparison() {
        assert_eq!(compare_versions("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
        assert_eq!(compare_versions("2.0", "1.9"), std::cmp::Ordering::Greater);
        assert_eq!(compare_versions("1.0.0", "1.0.0"), std::cmp::Ordering::Equal);
        assert_eq!(compare_versions("1.0rc1", "1.0"), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_solver_basic() {
        let mut solver = DepSolver::new();

        solver.add_available("nginx", PackageInfo {
            version: "1.26.0".into(),
            depends: vec!["pcre2".into(), "openssl".into()],
            provides: vec![],
            conflicts: vec![],
            repo: "core".into(),
            install_size: 2000000,
        });

        solver.add_available("pcre2", PackageInfo {
            version: "10.43".into(),
            depends: vec![],
            provides: vec![],
            conflicts: vec![],
            repo: "core".into(),
            install_size: 1000000,
        });

        solver.add_available("openssl", PackageInfo {
            version: "3.3.0".into(),
            depends: vec![],
            provides: vec![],
            conflicts: vec![],
            repo: "core".into(),
            install_size: 5000000,
        });

        let result = solver.solve(&["nginx".into()], true);
        assert_eq!(result.to_install.len(), 3);
    }
}
