use convert_case::{Case, Casing};
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

pub fn main() -> io::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut code = "".to_owned();
    let mods = ["scenes", "levels"]
        .iter()
        .flat_map(|&dir_name| {
            let dir_path = &manifest_dir.join("../../zf/").join(dir_name);
            let entries = fs::read_dir(dir_path)
                .expect(&format!("failed to read dir {dir_name}"))
                .filter_map(|entry| {
                    let path = entry.ok()?.path();
                    if OsStr::new("tscn") != path.extension()? {
                        return None;
                    };
                    Some(path)
                })
                .collect::<Vec<PathBuf>>();
            let paths = entries
                .iter()
                .map(|entry| {
                    let scene = entry
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_case(Case::ScreamingSnake);
                    let scene_path = entry
                        .to_str()
                        .unwrap()
                        .replace(dir_path.to_str().unwrap(), &format!("res://{dir_name}"));
                    format!(r#"    pub const {scene}: &str = "{scene_path}";"#)
                })
                .collect::<Vec<String>>()
                .join("\n");
            code.push_str(&fmt_mod(dir_name, &paths));
            entries
        })
        .map(|path| {
            let paths = fs::read_to_string(&path)?
                .lines()
                .filter_map(|line| {
                    let line = line.strip_prefix("[node ")?.strip_suffix("]")?;
                    let mut name: Option<&str> = None;
                    let mut parent: Option<&str> = None;
                    for part in line.split(' ') {
                        name = name.or(part
                            .strip_prefix("name=\"")
                            .and_then(|part| part.strip_suffix("\"")));
                        parent = parent.or(part
                            .strip_prefix("parent=\"")
                            .and_then(|part| part.strip_suffix("\"")));
                    }

                    if let (Some(name), Some(parent)) = (name, parent) {
                        let path = format!(
                            "/root/Scene/{}{}",
                            if parent == "." {
                                "".to_owned()
                            } else {
                                format!("{parent}/")
                            },
                            name
                        );
                        let name = name.to_case(Case::ScreamingSnake);
                        let line = format!(r#"    pub const {name}: &str = "{path}";"#);
                        return Some(line);
                    }
                    None
                })
                .collect::<Vec<String>>()
                .join("\n");
            let mod_name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .expect(&format!("invalid file: {:?}", path))
                .to_case(Case::Snake);
            let inner = fmt_mod(&mod_name, &paths);
            Ok(inner)
        })
        .collect::<io::Result<String>>()?;
    code.push_str(&mods);

    fs::write("./zf/src/path/path.rs", code).unwrap();
    Ok(())
}

fn fmt_mod(mod_name: &str, inner: &str) -> String {
    format!(
        r#"
#[rustfmt::skip]
#[allow(dead_code)]
pub mod {mod_name} {{
{inner}
}}

"#
    )
    .trim_start()
    .to_owned()
}
