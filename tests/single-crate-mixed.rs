use std::path::PathBuf;

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

    for section in ["src", "src/foo", "src/bar"] {
        std::fs::create_dir_all(root.join(section))?;
    }
    std::fs::write(
        root.join("Cargo.toml"),
        r#"[package]
name = "single-crate-mixed"
"#,
    )?;
    std::fs::write(root.join("src/lib.rs"), "mod foo;\nmod bar;\n")?;
    std::fs::write(root.join("src/foo.rs"), "mod bar;\n")?;
    std::fs::write(root.join("src/foo/bar.rs"), "\n")?;
    std::fs::write(root.join("src/bar/mod.rs"), "mod baz;\n")?;
    std::fs::write(root.join("src/bar/baz.rs"), "\n")?;
    Ok(root)
}

#[test]
fn should_move_the_files() {
    let root = prepare_directory().unwrap();
    let result = cargo_hilly::scan(root.clone()).unwrap();
    assert_eq!(result.len(), 1);

    let (src, dst) = result.first().unwrap();
    let src = src.strip_prefix(&root).unwrap();
    let dst = dst.strip_prefix(&root).unwrap();

    assert_eq!(src.to_string_lossy(), "src/foo.rs");
    assert_eq!(dst.to_string_lossy(), "src/foo/mod.rs");
    for (src, dst) in result {
        cargo_hilly::ModuleMigrator.migrate(&src, &dst).unwrap();
    }

    assert!(root.join("src/lib.rs").exists());
    assert!(!root.join("src/foo.rs").exists());
    assert!(root.join("src/foo/mod.rs").exists());
    assert!(root.join("src/foo/bar.rs").exists());
    assert!(root.join("src/bar/mod.rs").exists());
    assert!(root.join("src/bar/baz.rs").exists());
}
