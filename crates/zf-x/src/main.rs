use convert_case::{Case, Casing};
use std::{ffi::OsStr, fs, io, path::Path};

pub fn main() -> io::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mods = ["scene", "levels"]
        .iter()
        .flat_map(|dir| {
            let path = &manifest_dir.join("../../zf/").join(dir);
            fs::read_dir(path).expect(&format!("failed to read dir {dir}"))
        })
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if OsStr::new("tscn") != path.extension()? {
                return None;
            };
            Some(path)
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
            let inner = format!(
                r#"
#[rustfmt::skip]
#[allow(dead_code)]
pub mod {mod_name} {{
{paths}
}}

"#
            )
            .trim_start()
            .to_owned();
            Ok(inner)
        })
        .collect::<io::Result<String>>()?;

    fs::write("./zf/src/path/path.rs", mods).unwrap();
    Ok(())
}
