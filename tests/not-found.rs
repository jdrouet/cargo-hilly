use std::{io::ErrorKind, path::PathBuf};

fn prepare_directory() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let root = root.join("not-found");
    if root.exists() {
        std::fs::remove_dir_all(&root).unwrap();
    }
    root
}

#[test]
fn crates_scanner_should_find_nothing() {
    let root = prepare_directory();

    let mut scanner = cargo_hilly::CratesScanner::default();
    scanner.scan(root);
    assert!(scanner.into_inner().is_empty());
}

#[test]
fn scan_function_should_error() {
    let root = prepare_directory();

    let err = cargo_hilly::scan(root).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NotFound);
}
