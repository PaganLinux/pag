// Testy jednostkowe dla projektu Pag

// Testy dla modułu package
#[cfg(test)]
mod package_tests {
    use pag::package::PagPackage;
    
    #[test]
    fn test_package_creation() {
        let package = PagPackage::new("test-package", "1.0.0");
        assert_eq!(package.name, "test-package");
        assert_eq!(package.version, "1.0.0");
    }

    #[test]
    fn test_package_metadata() {
        let package = PagPackage::new("test-package", "1.0.0");
        assert_eq!(package.metadata().name, "test-package");
    }
}

// Testy dla modułu repo
#[cfg(test)]
mod repo_tests {
    use pag::repo::Repo;
    
    #[test]
    fn test_repo_initialization() {
        let repo = Repo::new("test-repo", "https://example.com/repo");
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.url, "https://example.com/repo");
    }

    #[test]
    fn test_repo_sync() {
        let repo = Repo::new("test-repo", "https://example.com/repo");
        assert!(repo.sync().is_ok());
    }
}

// Testy dla modułu crypto
#[cfg(test)]
mod crypto_tests {
    use pag::crypto::hash::HashAlgorithm;
    
    #[test]
    fn test_hash_generation() {
        let data = b"test-data";
        let hash = HashAlgorithm::Sha256.hash(data);
        assert_eq!(hash, "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    }

    #[test]
    fn test_gpg_verification() {
        let signature = "test-signature";
        let data = b"test-data";
        assert!(pag::crypto::gpg::verify_signature(signature, data).is_ok());
    }
}

// Testy dla modułu db
#[cfg(test)]
mod db_tests {
    use pag::db::LocalDb;
    
    #[test]
    fn test_db_connection() {
        let db = LocalDb::new(":memory:").unwrap();
        assert!(db.connection.is_ok());
    }

    #[test]
    fn test_db_query() {
        let db = LocalDb::new(":memory:").unwrap();
        let result = db.query("SELECT 1");
        assert!(result.is_ok());
    }
}

// Testy dla modułu download
#[cfg(test)]
mod download_tests {
    use pag::repo::download::DownloadManager;
    
    #[test]
    fn test_download_file() {
        let manager = DownloadManager::new();
        let url = "https://example.com/test-file.txt";
        let result = manager.download(url, "test-file.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_download_with_progress() {
        let manager = DownloadManager::new();
        let url = "https://example.com/large-file.txt";
        let result = manager.download_with_progress(url, "large-file.txt");
        assert!(result.is_ok());
    }
}

// Testy dla modułu delta
#[cfg(test)]
mod delta_tests {
    use pag::repo::delta::DeltaUpdate;
    
    #[test]
    fn test_delta_update_creation() {
        let delta = DeltaUpdate::new("1.0.0", "1.0.1", "patch.diff");
        assert_eq!(delta.source_version, "1.0.0");
        assert_eq!(delta.target_version, "1.0.1");
        assert_eq!(delta.patch_file, "patch.diff");
    }

    #[test]
    fn test_delta_patch_application() {
        let delta = DeltaUpdate::new("1.0.0", "1.0.1", "patch.diff");
        assert!(delta.apply_patch("source.txt", "target.txt").is_ok());
    }
}
