use std::fs;

pub fn main() {
    let theme = include_str!("../../../zf/scene/space.tscn");
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
                let name = const_case(name.to_owned());
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
    fs::write("./zf/src/path/path.rs", out.trim_start()).unwrap();
}

fn const_case(text: String) -> String {
    #[derive(PartialEq, Clone, Copy, Debug)]
    enum Case {
        Lower,
        Upper,
        Num,
        UnderScore,
        Other,
    }

    use Case::*;
    let mut last: Option<Case> = None;
    text.chars()
        .map(|c| {
            let ty = match c {
                'a'..='z' => Lower,
                'A'..='Z' => Upper,
                '0'..='9' => Num,
                '-' | '_' => UnderScore,
                _ => Other,
            };
            let last_ty = last.replace(ty);
            match ty {
                Lower => c.to_uppercase().to_string(),
                upper_or_num @ (Upper | Num) => last_ty
                    .and_then(|ty| ty.ne(&upper_or_num).then_some(format!("_{c}")))
                    .unwrap_or(c.to_string()),
                UnderScore => "_".to_owned(),
                Other => "".to_owned(),
            }
        })
        .collect()
}
