use std::path::PathBuf;

#[test]
fn should_return_the_files_to_move() {
    let root = PathBuf::new().join("tests").join("resources").join("mixed");
    let mut result = cargo_hilly::scan(root.clone()).unwrap();
    assert_eq!(result.len(), 1);
    let (src, dst) = result.pop().unwrap();
    let src = src.strip_prefix(&root).unwrap();
    let dst = dst.strip_prefix(&root).unwrap();
    assert_eq!(src.to_string_lossy(), "src/foo.rs");
    assert_eq!(dst.to_string_lossy(), "src/foo/mod.rs");
}
