#[cfg(test)]
mod tests {
    use organize::file_ops::find_nonconflicting_path;
    use std::ffi::OsStr;
    use std::fs::File;

    #[test]
    fn test_find_nonconflicting_path_adds_suffix() {
        let temp = tempfile::tempdir().unwrap();
        let base_path = temp.path();

        let original_name = "sample.txt";
        let file_path = base_path.join(original_name);
        File::create(&file_path).unwrap();

        let candidate = find_nonconflicting_path(base_path, OsStr::new(original_name));
        assert!(
            candidate
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("sample (1)")
        );
    }

    #[test]
    fn test_find_nonconflicting_path_direct_pass_through() {
        let temp = tempfile::tempdir().unwrap();
        let base_path = temp.path();

        let candidate = find_nonconflicting_path(base_path, OsStr::new("newfile.txt"));
        assert_eq!(candidate, base_path.join("newfile.txt"));
    }
}
