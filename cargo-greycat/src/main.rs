use serde::Deserialize;

use std::borrow::Cow;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const GREYCAT_HOST_TARGET: &str = "x64-windows";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const GREYCAT_HOST_TARGET: &str = "x64-apple";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const GREYCAT_HOST_TARGET: &str = "arm64-apple";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const GREYCAT_HOST_TARGET: &str = "x64-linux";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub const GREYCAT_HOST_TARGET: &str = "arm64-linux";

#[cfg(not(any(
    all(target_os = "windows", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "aarch64"),
)))]
pub const GREYCAT_HOST_TARGET: &str = "unsupported";

#[derive(Debug, Deserialize)]
struct Metadata {
    workspace_root: PathBuf,
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    manifest_path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "reason")]
pub enum CargoMessage {
    #[serde(rename = "compiler-artifact")]
    CompilerArtifact(CompilerArtifact),

    #[serde(rename = "build-finished")]
    BuildFinished(BuildFinished),

    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct CompilerArtifact {
    pub manifest_path: PathBuf,
    pub filenames: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BuildFinished {
    pub success: bool,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(|v| v.as_str()) {
        Some("build") => greycat_build(&args[1..]),
        Some("install") => greycat_install(&args[1..]),
        Some("codegen") => greycat_codegen(&args[1..]),
        Some(cmd) => {
            eprintln!("unknown command: {}", cmd);
            exit(1);
        }
        None => greycat_build(&args),
    }
}

// top-level build command
fn greycat_build(args: &[String]) {
    let crate_dir = find_crate_root();
    let greycat_target = infer_target_from_args_or_env(args);
    let metadata = get_metadata();
    let package = metadata
        .packages
        .iter()
        .find(|p| p.manifest_path.starts_with(&crate_dir))
        .expect("unable to find crate in metadata");

    // pre-build phase
    greycat_pre_build(&crate_dir, &greycat_target);
    // cargo build phase: propagate arguments
    let artifacts = cargo_build(&package.manifest_path, args);
    // post-build phase
    greycat_post_build(&metadata.workspace_root, &crate_dir, &artifacts);
}

fn greycat_pre_build(crate_dir: &Path, greycat_target: &str) {
    // run_greycat(crate_dir, &["install"]);
    // run_greycat(crate_dir, &["codegen", "rust"]);

    // greycat install with target and --force
    let status = Command::new("greycat")
        .env("GREYCAT_TARGET", greycat_target)
        .current_dir(crate_dir)
        .arg("install")
        .arg("--force")
        .status()
        .expect("failed to run greycat install --force");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    // set platform-specific linker args (for windows/macOS)
    if greycat_target.ends_with("-windows") {
        let lib_path = fs::canonicalize(crate_dir.join("lib/greycat_sdk_headers.a"))
            .expect("unable to find lib/greycat_sdk_headers.a");
        println!("cargo:rustc-link-arg=-Wl,--whole-archive");
        println!("cargo:rustc-link-arg={}", lib_path.display());
        println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
    } else if greycat_target.ends_with("-apple") {
        println!("cargo:rustc-link-arg=-Wl,-undefined");
        println!("cargo:rustc-link-arg=-Wl,dynamic_lookup");
    }
}

fn cargo_build(manifest_path: &Path, args: &[String]) -> Vec<String> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build").args(args).arg("--message-format=json");
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to run cargo build");
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let mut artifacts = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<CargoMessage>(&line) {
            Ok(CargoMessage::CompilerArtifact(artifact)) => {
                if artifact.manifest_path == manifest_path {
                    for file in artifact.filenames.into_iter().filter(|filepath| {
                        filepath.ends_with(".so")
                            || filepath.ends_with(".dll")
                            || filepath.ends_with(".dylib")
                    }) {
                        artifacts.push(file);
                    }
                }
            }
            Ok(CargoMessage::BuildFinished(info)) => {
                if !info.success {
                    eprintln!("build failed");
                    std::process::exit(1);
                }
            }
            _ => {}
        }
    }

    let status = child.wait().unwrap();
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    artifacts
}

fn greycat_post_build(ws_root: &Path, crate_dir: &Path, artifacts: &[String]) {
    let crate_name = get_crate_name(crate_dir);

    let dest_dir = crate_dir.join("lib").join(&crate_name);
    fs::create_dir_all(&dest_dir).expect("failed to create destination directory");

    let dest_path = dest_dir.join(format!("{}.gclib", crate_name));
    for artifact in artifacts {
        let artifact_path = Path::new(artifact);
        // compute relative artifact path
        let rel_artifact = artifact_path.strip_prefix(ws_root).unwrap_or(artifact_path);

        // compute relative dest path
        let rel_dest = dest_path.strip_prefix(ws_root).unwrap_or(&dest_path);

        fs::copy(artifact_path, &dest_path)
            .unwrap_or_else(|_| panic!("failed to copy {artifact} to {}", dest_path.display()));

        println!(
            "[greycat] {} -> {}",
            rel_artifact.display(),
            rel_dest.display()
        );
    }
}

#[allow(unused)]
fn run_greycat(current_dir: &Path, args: &[&str]) {
    let status = Command::new("greycat")
        .current_dir(current_dir)
        .args(args)
        .status()
        .unwrap_or_else(|_| panic!("failed to run greycat {}", args.join(" ")));
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

fn get_crate_name(crate_dir: &Path) -> String {
    let manifest =
        fs::read_to_string(crate_dir.join("Cargo.toml")).expect("failed to read Cargo.toml");
    for line in manifest.lines() {
        if let Some(name) = line.strip_prefix("name = ") {
            return name.trim_matches(|c| c == '"' || c == ' ').to_string();
        }
    }
    panic!("could not find crate name in Cargo.toml");
}

fn greycat_install(_args: &[String]) {
    todo!()
}

fn greycat_codegen(args: &[String]) {
    match args.first().map(|v| v.as_str()) {
        Some("rust") => println!("greycat codegen rust not yet implemented in CLI"),
        Some(cmd) => {
            eprintln!("unknown codegen target: {}", cmd);
            exit(1);
        }
        None => {
            eprintln!("no codegen target provided");
            exit(1);
        }
    }
}

fn infer_target_from_args_or_env(args: &[String]) -> Cow<'static, str> {
    // 1. check for explicit `--target=foo`
    if let Some(pos) = args.iter().position(|arg| arg.starts_with("--target=")) {
        let val = args[pos].split_once('=').unwrap().1.to_string();
        return Cow::Owned(val);
    }

    // 2. check for `--target foo`
    if let Some(pos) = args.iter().position(|arg| arg == "--target") {
        if let Some(val) = args.get(pos + 1) {
            return Cow::Owned(val.clone());
        }
    }

    // 3. check for CARGO_BUILD_TARGET env var
    if let Ok(val) = std::env::var("CARGO_BUILD_TARGET") {
        return Cow::Owned(val);
    }

    // 4. check for GREYCAT_TARGET
    if let Ok(val) = env::var("GREYCAT_TARGET") {
        return Cow::Owned(val);
    }

    // 5. fallback to the host compiled target
    if GREYCAT_HOST_TARGET == "unsupported" {
        panic!("unsupported host platform for greycat target");
    }

    Cow::Borrowed(GREYCAT_HOST_TARGET)
}

fn find_crate_root() -> PathBuf {
    let mut dir = env::current_dir().unwrap();
    loop {
        if dir.join("Cargo.toml").exists() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    panic!("unable to find a Cargo.toml")
}

fn get_metadata() -> Metadata {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version=1")
        .output()
        .expect("failed to run cargo metadata");

    let stdout = String::from_utf8(output.stdout).unwrap();
    serde_json::from_str::<Metadata>(&stdout).expect("unable to deserialize cargo metadata json")
}
