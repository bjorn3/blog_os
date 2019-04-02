use std::{env, fs::File, io::Write, path::Path, fmt};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_tests.rs");
    let mut tests = File::create(&dest_path).unwrap();

    let path = Path::new("tests/integration_tests/src/bin");
    for entry in path.read_dir().expect("failed to read integration tests crate") {
        let entry = entry.expect("failed to read dir entry");
        assert!(entry.file_type().unwrap().is_file());
        let test_path = entry.path();
        let test_name = test_path.file_stem().expect("no file stem").to_os_string().into_string().expect("file name not valid utf8");

        let content = format!(r#"
#[test]
fn {test_name_escaped}() {{
    run_test("{test_name}");
}}
"#, test_name_escaped = test_name.replace("-", "_"), test_name = test_name);

        tests.write_all(content.as_bytes()).expect("failed to write test");

        println!("cargo:rerun-if-changed={}", test_path.display());
    }

    println!("cargo:rustc-env=GENERATED_TESTS={}", dest_path.display());
    println!("cargo:rerun-if-changed=build.rs");
}
