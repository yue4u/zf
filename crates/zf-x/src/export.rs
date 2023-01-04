use std::{fs, io, path::PathBuf, process::Command};

fn main() -> io::Result<()> {
    let root = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let build_dir = root.join("build");
    let gd_dir = root.join("zf");

    fs::remove_dir_all(&build_dir)?;
    fs::create_dir_all(&build_dir)?;

    Command::new("godot")
        .current_dir(gd_dir)
        .args(["--no-window", "--export", "Linux/X11", "--path", "."])
        .spawn()?
        .wait()?;

    Ok(())
}
