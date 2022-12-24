use convert_case::{Case, Casing};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

pub fn main() -> io::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let gd_dir = manifest_dir.join("../../zf/").canonicalize()?;
    let mut code = "use gdnative::prelude::{FromVariant, ToVariant};\n\n".to_owned();

    let mods = ["scenes", "levels"]
        .iter()
        .flat_map(|&dir_name| {
            let dir_path = &gd_dir.join(dir_name);
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
            let mut level_inner = vec![];
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
                    let level_enum_varient = fmt_path_name(&scene);
                    level_inner.push(level_enum_varient.to_case(Case::UpperCamel));
                    fmt_path(&scene, &scene_path)
                })
                .collect::<Vec<String>>()
                .join("\n");
            code.push_str(&fmt_mod(dir_name, &paths));

            if dir_name == "levels" {
                let mut level_enum_inner = level_inner.clone();
                level_enum_inner.push("Unknown".to_owned());

                code.push_str(&fmt_enum(
                    "LevelName",
                    &level_enum_inner
                        .into_iter()
                        .map(|v| format!("    {},", v.to_case(Case::UpperCamel)))
                        .collect::<Vec<String>>()
                        .join("\n"),
                ));

                let from_path_inner = &level_inner
                    .iter()
                    .map(|v| {
                        format!(
                            "            levels::{} => LevelName::{},",
                            v.to_case(Case::ScreamingSnake),
                            v.to_case(Case::UpperCamel)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                code.push_str(&format!(
                    r#"
impl LevelName {{
    pub fn from_path(value: &str) -> Self {{
        match value {{
{from_path_inner}
            _ => LevelName::Unknown,
        }}
    }}
}}

"#
                ));

                let path_inner = &level_inner
                    .iter()
                    .map(|v| {
                        format!(
                            "            LevelName::{} => levels::{},",
                            v.to_case(Case::UpperCamel),
                            v.to_case(Case::ScreamingSnake),
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let display_inner = &level_inner
                    .iter()
                    .map(|v| {
                        format!(
                            "            LevelName::{} => \"{}\",",
                            v.to_case(Case::UpperCamel),
                            v.to_case(Case::UpperCamel),
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let from_inner = &level_inner
                    .iter()
                    .map(|v| {
                        format!(
                            "            \"{}\" => LevelName::{},",
                            v.to_case(Case::UpperCamel),
                            v.to_case(Case::UpperCamel),
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                code.push_str(&format!(
                    r#"impl LevelName {{
    pub fn path(&self) -> &'static str {{
        match self {{
{path_inner}
            LevelName::Unknown => unreachable!(),
        }}
    }}
}}

impl LevelName {{
    pub fn from(name: &str) -> LevelName {{
        match name {{
{from_inner}
            _ => LevelName::Unknown,
        }}
    }}

    pub fn as_str(&self) -> &str {{
        match &self {{
{display_inner}
            _ => "Unknown",
        }}
    }}
}}

impl std::fmt::Display for LevelName {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        f.write_str(self.as_str())
    }}
}}
"#
                ));
            }
            entries //.into_iter().map(move |e| (e, dir_name))
        })
        .map(|path| {
            let mut seen = HashMap::<String, u32>::new();
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
                    let base = if path.ends_with("AutoLoad.tscn") {
                        "/AutoLoad"
                    } else {
                        "/Scene"
                    };

                    if let (Some(name), Some(parent)) = (name, parent) {
                        let node_path = if path.ends_with("PlayerHealthBar.tscn") {
                            format!(
                                ".{}{}",
                                if parent == "." {
                                    "/".to_owned()
                                } else {
                                    format!("/{parent}/")
                                },
                                name
                            )
                        } else {
                            format!(
                                "/root{}{}{}",
                                base,
                                if parent == "." {
                                    "/".to_owned()
                                } else {
                                    format!("/{parent}/")
                                },
                                name
                            )
                        };
                        let name = name.to_case(Case::ScreamingSnake);
                        *seen.entry(name.clone()).or_insert(0) += 1;
                        let name = if seen[&name] > 1 {
                            format!("{}_{}", name, seen[&name] - 1)
                        } else {
                            name
                        };
                        let line = fmt_path(name, &node_path);
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

    assets(&mut code, &gd_dir, &gd_dir.join("assets/"));

    fs::write(manifest_dir.join("../zf/src/refs/path.rs"), code).unwrap();
    Ok(())
}

fn assets(code: &mut String, gd_dir: &PathBuf, assets_dir: &PathBuf) {
    let inner = fs::read_dir(assets_dir)
        .expect("failed to read assets dir")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() || OsStr::new("import") == path.extension()? {
                return None;
            };
            let name = path.file_name().unwrap().to_string_lossy();
            let path = path
                .to_string_lossy()
                .replace(&gd_dir.to_string_lossy().to_string(), "res:/");
            Some(fmt_path(name, &path))
        })
        .collect::<Vec<String>>();
    code.push_str(&fmt_mod("assets", &inner.join("\n")));
}

fn fmt_path_name(name: impl ToString) -> String {
    name.to_string()
        .replace('.', "_")
        .to_case(Case::ScreamingSnake)
}

fn fmt_path(name: impl ToString, path: &str) -> String {
    let name = fmt_path_name(name);
    format!(r#"    pub const {name}: &str = "{path}";"#)
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

fn fmt_enum(enum_name: &str, inner: &str) -> String {
    format!(
        r#"
#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, ToVariant, FromVariant)]
pub enum {enum_name} {{
{inner}
}}

"#
    )
    .trim_start()
    .to_owned()
}
