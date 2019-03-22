use std::{
    env,
    path::{Path, PathBuf},
    process::{self, Command},
};

const BOOTLOADER_NAME: &str = "bootloader";

fn main() {
    let manifest_path = env::var("CRATE_MANIFEST_PATH").expect("CRATE_MANIFEST_PATH not set");
    let out_dir = PathBuf::from(env::var("CRATE_OUT_DIR").expect("CRATE_OUT_DIR not set"));
    let kernel_path = out_dir.join("blog_os");

    let metadata = {
        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.manifest_path(manifest_path);
        cmd.exec().expect("failed to run cargo metadata")
    };

    let bootloader_pkg = metadata
        .packages
        .iter()
        .find(|p| p.name == BOOTLOADER_NAME)
        .expect("bootloader dependency not found");
    let bootloader_target = bootloader_pkg
        .manifest_path
        .parent()
        .expect("bootloader manifest has no parent")
        .join("x86_64-bootloader.json");
    let bootloader_features = metadata
        .resolve
        .expect("no metadata.resolve info")
        .nodes
        .iter()
        .find(|n| n.id == bootloader_pkg.id)
        .expect("bootloader not found in resolve graph")
        .features.clone();

    // build bootloader
    println!("Building bootloader");
    let mut cmd = process::Command::new("cargo");
    cmd.arg("xbuild");
    cmd.arg("--manifest-path");
    cmd.arg(&bootloader_pkg.manifest_path);
    cmd.arg("--target").arg(bootloader_target);
    cmd.arg("--target-dir").arg(out_dir.join("bootloader"));
    cmd.arg("--features")
        .arg(bootloader_features.as_slice().join(" "));
    cmd.arg("--release");
    cmd.env("KERNEL", kernel_path);
    let exit_status = cmd.status().expect("failed to execute bootloader builder");
    assert!(exit_status.success(), "failed to build bootloader");

    let bootloader_elf_path = out_dir
        .join("bootloader")
        .join("x86_64-bootloader")
        .join("release")
        .join("bootloader");
    // TODO add crate name: bootimage-{}.bin
    let bootloader_bin_path = out_dir.join(format!("bootimage.bin"));

    let bin_dir = BinDir::new();
    let objcopy = bin_dir
        .tool(&LlvmTool::tool_name("objcopy"))
        .expect("llvm-objcopy not found in llvm-tools");

    // convert bootloader to binary
    let mut cmd = Command::new(objcopy.path());
    cmd.arg("-I").arg("elf64-x86-64");
    cmd.arg("-O").arg("binary");
    cmd.arg("--binary-architecture=i386:x86-64");
    cmd.arg(&bootloader_elf_path);
    cmd.arg(&bootloader_bin_path);
    let exit_status = cmd.status().expect("failed to run objcopy");
    if !exit_status.success() {
        eprintln!("Error: Running objcopy failed");
        process::exit(1);
    }
}

#[derive(Debug)]
struct BinDir {
    bin_dir: PathBuf,
}

impl BinDir {
    fn new() -> Self {
        let example_tool_name = LlvmTool::tool_name("objdump");
        let output = Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()
            .expect("failed to print sysroot");
        if !output.status.success() {
            eprintln!("Failed to execute `rustc --print sysroot`");
            eprintln!(
                "Stderr: {}",
                String::from_utf8(output.stderr).expect("error not valid unicode")
            );
            process::exit(1);
        }

        let sysroot = PathBuf::from(
            String::from_utf8(output.stdout)
                .expect("sysroot not valid unicode")
                .trim(),
        );

        let rustlib = sysroot.join("lib").join("rustlib");
        for entry in rustlib.read_dir().expect("read_dir on sysroot dir failed") {
            let bin_dir = entry
                .expect("failed to read sysroot dir entry")
                .path()
                .join("bin");
            let tool_path = bin_dir.join(&example_tool_name);
            if tool_path.exists() {
                return Self { bin_dir };
            }
        }

        eprintln!("Error: llvm-tools not found");
        eprintln!("Maybe the rustup component `llvm-tools-preview` is missing?");
        eprintln!("  Install it through: `rustup component add llvm-tools-preview`");
        process::exit(1);
    }

    fn tool(&self, tool_name: &str) -> Option<LlvmTool> {
        let tool_path = self.bin_dir.join(&tool_name);

        if tool_path.exists() {
            Some(LlvmTool {
                name: tool_name.to_owned(),
                path: tool_path,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LlvmTool {
    name: String,
    path: PathBuf,
}

impl LlvmTool {
    fn path(&self) -> &Path {
        &self.path
    }

    #[cfg(target_os = "windows")]
    fn tool_name(tool: &str) -> String {
        format!("llvm-{}.exe", tool)
    }

    #[cfg(not(target_os = "windows"))]
    fn tool_name(tool: &str) -> String {
        format!("llvm-{}", tool)
    }
}
