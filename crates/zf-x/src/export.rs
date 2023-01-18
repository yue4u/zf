use std::{
    fs,
    io::{self, ErrorKind},
    path::PathBuf,
    process::Command,
};

fn main() -> io::Result<()> {
    let root = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
    let build_dir = root.join("build");
    let gd_dir = root.join("zf");

    match fs::remove_dir_all(&build_dir) {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        e => return e,
    };

    fs::create_dir_all(&build_dir)?;

    #[cfg(target_os = "linux")]
    Command::new("godot")
        .current_dir(gd_dir)
        .args(["--no-window", "--export", "Linux/X11", "--path", "."])
        .spawn()?
        .wait()?;

    #[cfg(not(target_os = "linux"))]
    eprintln!("godot cli is not avalible, export directly!");

    Ok(())
}
