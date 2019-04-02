use std::path::Path;

include!(env!("GENERATED_TESTS"));

fn run_test(test_name: &str) {
    let integration_test_crate = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/integration_tests");

    let mut cmd = std::process::Command::new("bootimage");
    cmd.arg("run");
    cmd.arg("--bin").arg(test_name);
    cmd.arg("--quiet");
    cmd.current_dir(integration_test_crate);
    let output = cmd.output().expect("failed to run bootimage");
    match output.status.code() {
        None => panic!("no exit code"),
        Some(5) => {},
        Some(7) => {
            // test failed
            panic!("Failed:\n{}", String::from_utf8_lossy(&output.stdout))
        }
        Some(1) => {
            panic!("Error:\n{}", String::from_utf8_lossy(&output.stderr))
        },
        Some(x) => panic!("invalid exit code {}", x),
    }
}
