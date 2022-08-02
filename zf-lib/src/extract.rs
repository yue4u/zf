use std::fs;

pub fn main() {
    let theme = include_str!("../../zf/scene/space.tscn");
    let out = theme
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
                let name = name.replace("-", "_").to_uppercase();
                let line = format!(r#"    pub const {name}: &str = "{path}";"#);
                return Some(line);
            }
            None
        })
        .collect::<Vec<String>>()
        .join("\n");
    let out = format!(
        r#"
#[rustfmt::skip]
#[allow(dead_code)]
pub mod space {{
{out}
}}
"#
    );
    fs::write("./src/common/path.rs", out.trim_start()).unwrap();
}
