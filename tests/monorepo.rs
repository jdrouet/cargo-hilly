use std::path::{Path, PathBuf};

/// Creates a temporary directory
///
/// /Cargo.toml
/// /src/lib.rs
/// /src/foo.rs
/// /src/foo/bar.rs
/// /src/bar/mod.rs
/// /src/bar/baz.rs
fn prepare_directory() -> std::io::Result<PathBuf> {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let root = root.join("single-crate-mixed");
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }

    std::fs::create_dir_all(&root)?;
    std::fs::write(root.join("Cargo.toml"), "")?;
    create_crate(&root.join("packages/first"))?;
    create_crate(&root.join("packages/second"))?;

    Ok(root)
}

fn create_crate(root: &Path) -> std::io::Result<()> {
    for section in ["src", "src/foo", "src/bar"] {
        std::fs::create_dir_all(&root.join(section))?;
    }
    std::fs::write(
        root.join("Cargo.toml"),
        r#"[package]
name = "any-name"
"#,
    )?;
    std::fs::write(root.join("src/lib.rs"), "mod foo;\nmod bar;\n")?;
    std::fs::write(root.join("src/foo.rs"), "mod bar;\n")?;
    std::fs::write(root.join("src/foo/bar.rs"), "\n")?;
    std::fs::write(root.join("src/bar/mod.rs"), "mod baz;\n")?;
    std::fs::write(root.join("src/bar/baz.rs"), "\n")?;
    std::fs::write(root.join("src/bar/no-prefix"), "\n")?;
    std::fs::write(root.join("src/some-data.json"), "\n")?;
    Ok(())
}

#[test]
fn should_move_the_files() {
    let root = prepare_directory().unwrap();
    let result = cargo_hilly::scan(root.clone()).unwrap();
    assert_eq!(result.len(), 2);

    for (src, dst) in result {
        cargo_hilly::ModuleMigrator.migrate(&src, &dst).unwrap();
    }

    for subdir in ["packages/first", "packages/second"] {
        let root = root.join(subdir);
        assert!(root.join("src/lib.rs").exists());
        assert!(!root.join("src/foo.rs").exists());
        assert!(root.join("src/foo/mod.rs").exists());
        assert!(root.join("src/foo/bar.rs").exists());
        assert!(root.join("src/bar/mod.rs").exists());
        assert!(root.join("src/bar/baz.rs").exists());
    }
}
