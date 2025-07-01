use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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

/// Finds all the crates in the current directory
fn find_crates(collector: &mut Vec<PathBuf>, base: PathBuf) {
    if base.join("Cargo.toml").is_file() && base.join("src").is_dir() {
        collector.push(base.clone());
    }

    std::fs::read_dir(&base)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| {
            entry.is_dir()
                && entry
                    .file_name().is_none_or(|name| name.to_string_lossy() != "src")
        })
        .for_each(|entry| find_crates(collector, entry));
}

/// List all the files that can be moved
fn find_candidate_modules(collector: &mut Vec<(PathBuf, PathBuf)>, base: &Path) {
    std::fs::read_dir(base)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .for_each(|path| {
            if path.is_dir() {
                find_candidate_modules(collector, &path);
            } else if let Some(name) = get_stem_rs(&path) {
                let sibling_dir = path.with_file_name(name);

                if sibling_dir.is_dir() {
                    let mod_rs_path = sibling_dir.join("mod.rs");
                    if !mod_rs_path.exists() {
                        collector.push((path, mod_rs_path));
                    }
                }
            }
        });
}

/// Move the files
fn migrate_module(dry_run: bool, from: &Path, to: &Path) -> std::io::Result<()> {
    println!("🚀 {from:?} → {to:?}");
    if !dry_run {
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::rename(from, to)?;
    }
    Ok(())
}

#[derive(Debug, Default)]
struct Args {
    dry_run: bool,
    check: bool,
}

impl Args {
    fn with_arg(mut self, arg: String) -> Self {
        match arg.as_str() {
            "--dry-run" => {
                self.dry_run = true;
            }
            "--check" => {
                self.check = true;
            }
            _ => {}
        };
        self
    }

    fn from_args() -> Self {
        std::env::args().fold(Self::default(), |acc, arg| acc.with_arg(arg))
    }

    fn dry_run(&self) -> bool {
        self.dry_run || self.check
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let crate_root = std::env::current_dir()?;

    let mut crates = Vec::new();
    find_crates(&mut crates, crate_root);

    if crates.is_empty() {
        eprintln!("⚠️ no crates found in the current directory");
        std::process::exit(1);
    }

    let mut candidates = Vec::new();
    for crate_path in crates {
        let src_path = crate_path.join("src");
        find_candidate_modules(&mut candidates, &src_path);
    }

    if candidates.is_empty() {
        println!("✅ No modules to migrate.");
        return Ok(());
    }

    for (from, to) in &candidates {
        migrate_module(args.dry_run(), from, to)?;
    }

    if args.check {
        eprintln!("⚠️ {} module(s) need to be migrated", candidates.len());
        std::process::exit(1);
    } else {
        println!("✅ Migrated {} module(s).", candidates.len());
    }
    Ok(())
}
