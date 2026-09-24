use std::process::exit;

use image::{imageops::FilterType, DynamicImage};
use showie::Trim;

use crate::{list::List, Data};

pub struct Trainer {
    pub path: String,
    pub name: String,
    pub sprite: DynamicImage,
}

impl Trainer {
    pub fn new(arg: String, list: &List, scale: f32) -> Self {
        if arg.eq_ignore_ascii_case("random") {
            return Self::load(list.random().to_owned(), list, scale);
        }

        let cleaned = sanitize(&arg);

        if list.get(&cleaned).is_none() {
            eprintln!("trainer not found: {arg}");
            suggest(&cleaned, list);
            exit(1);
        }

        Self::load(cleaned, list, scale)
    }

    fn load(name: String, list: &List, scale: f32) -> Self {
        let path = format!("{name}.png");

        let bytes = Data::get(&path)
            .unwrap_or_else(|| {
                eprintln!("trainer not found: {name}");
                exit(1)
            })
            .data
            .into_owned();

        let sprite = image::load_from_memory(&bytes).unwrap().trim();
        let sprite = downscale(sprite, scale);

        Self {
            path,
            name: list.format_name(&name),
            sprite,
        }
    }
}

fn downscale(sprite: DynamicImage, scale: f32) -> DynamicImage {
    let scale = scale.clamp(0.1, 4.0);

    if (scale - 1.0).abs() < f32::EPSILON {
        return sprite;
    }

    let width = ((sprite.width() as f32) * scale).round().max(1.0) as u32;
    let height = ((sprite.height() as f32) * scale).round().max(1.0) as u32;

    sprite.resize(width, height, FilterType::Nearest)
}

fn sanitize(name: &str) -> String {
    name.to_lowercase()
        .replace([' ', '_'], "-")
        .replace(['.', '\'', ':'], "")
        .trim()
        .to_owned()
}

fn suggest(input: &str, list: &List) {
    let mut close: Vec<&String> = list
        .iter()
        .filter(|name| levenshtein(name, input) <= 2)
        .take(5)
        .collect();

    if close.is_empty() {
        close = list
            .iter()
            .filter(|name| name.contains(input))
            .take(5)
            .collect();
    }

    if !close.is_empty() {
        let names = close
            .iter()
            .map(|n| n.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!("did you mean: {names}");
    } else {
        eprintln!("run `poketrainers --list` to see all available trainers");
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0; b.len() + 1];

    for (i, ca) in a.iter().enumerate() {
        curr[0] = i + 1;

        for (j, cb) in b.iter().enumerate() {
            curr[j + 1] = (prev[j + 1] + 1)
                .min(curr[j] + 1)
                .min(prev[j] + usize::from(ca != cb));
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    prev[b.len()]
}