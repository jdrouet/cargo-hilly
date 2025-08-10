use std::path::{Path, PathBuf};

use std::ffi::OsStr;

/// Checks if the file is a rust file
fn is_rust_file(file: &Path) -> bool {
    file.extension().is_some_and(|ext| ext == "rs")
}

/// Returns the filename without the extension
fn get_stem_rs(file: &Path) -> Option<&OsStr> {
    if is_rust_file(file) {
        file.file_stem()
            .filter(|name| name.to_string_lossy() != "mod")
    } else {
        None
    }
}

#[derive(Debug, Default)]
pub struct CratesScanner(Vec<PathBuf>);

impl CratesScanner {
    /// Finds all the crates in the given directory
    pub fn scan(&mut self, base: PathBuf) {
        if base.join("Cargo.toml").is_file() && base.join("src").is_dir() {
            self.0.push(base.clone());
        }

        std::fs::read_dir(&base)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|entry| {
                entry.is_dir()
                    && entry
                        .file_name()
                        .is_none_or(|name| name.to_string_lossy() != "src")
            })
            .for_each(|entry| self.scan(entry));
    }

    pub fn into_inner(self) -> Vec<PathBuf> {
        self.0
    }
}

#[derive(Debug, Default)]
pub struct FlatModuleScanner(Vec<(PathBuf, PathBuf)>);

impl FlatModuleScanner {
    /// Scan for all the flat modules that can be moved
    fn scan(&mut self, base: &Path) {
        std::fs::read_dir(base)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .for_each(|path| {
                if path.is_dir() {
                    self.scan(&path);
                } else if let Some(name) = get_stem_rs(&path) {
                    let sibling_dir = path.with_file_name(name);

                    if sibling_dir.is_dir() {
                        let mod_rs_path = sibling_dir.join("mod.rs");
                        if !mod_rs_path.exists() {
                            self.0.push((path, mod_rs_path));
                        }
                    }
                }
            });
    }

    pub fn scan_crates(&mut self, crates: &[PathBuf]) {
        for path in crates {
            let src_path = path.join("src");
            self.scan(&src_path);
        }
    }

    pub fn into_inner(self) -> Vec<(PathBuf, PathBuf)> {
        self.0
    }
}

pub fn scan(base: PathBuf) -> std::io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut crates = CratesScanner::default();
    crates.scan(base);
    let crates = crates.into_inner();

    if crates.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no crates found in directory",
        ));
    }

    let mut modules = FlatModuleScanner::default();
    modules.scan_crates(&crates);

    Ok(modules.into_inner())
}

pub struct ModuleMigrator;

impl ModuleMigrator {
    pub fn migrate(&self, src: &Path, dst: &Path) -> std::io::Result<()> {
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::rename(src, dst)?;
        Ok(())
    }
}
