#![feature(iter_array_chunks)]

use std::{fs, io, str::FromStr};

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Curve3DPoints(Vec<Point>);

const PREFIX: &'static str = "\"points\": PoolVector3Array(";

impl FromStr for Curve3DPoints {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .lines()
            .find(|l| l.starts_with(PREFIX))
            .map(|line| {
                let array = line.strip_prefix(PREFIX)?.strip_suffix(" ),")?;
                let pos = array
                    .split(',')
                    .into_iter()
                    .map_while(|num| num.trim().parse::<f32>().ok())
                    .array_chunks::<3>()
                    .map(|[x, y, z]| Point { x, y, z })
                    .collect::<Vec<Point>>();
                Some(Curve3DPoints(pos))
            })
            .unwrap()
            .unwrap();
        Ok(points)
    }
}

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("scene path");
    let scene = fs::read_to_string(path)?;
    let curve: Curve3DPoints = scene.parse()?;
    curve
        .0
        .iter()
        .enumerate()
        .filter(|i| (i.0 + 1) % 3 == 0)
        .enumerate()
        .for_each(|(i, (_, p))| {
            println!(
                r#"
[node name="TargetPoint_{}" parent="Level" index="{}" instance=ExtResource( 1 )]
transform = Transform( 1, 0, 0, 0, 1, 0, 0, 0, 1, {}, {}, {} )
"#,
                i,
                i + 100,
                p.x,
                p.y,
                p.z
            );
        });
    Ok(())
}
