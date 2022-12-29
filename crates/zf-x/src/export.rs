use std::{path::PathBuf, process::Command};

fn main() {
    let root = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));

    let build_dir = root.join("build");
    let gd_dir = root.join("zf");

    Command::new("rm")
        .args(["-rf", build_dir.to_str().unwrap()])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::fs::create_dir_all(build_dir).unwrap();

    Command::new("godot")
        .current_dir(gd_dir)
        .args(["--no-window", "--export", "Linux/X11", "--path", "."])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
