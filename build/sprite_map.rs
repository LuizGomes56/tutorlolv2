// use super::*;
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, fs, path::Path};

// #[derive(Deserialize, Serialize)]
// struct SpriteInner {
//     f: u8,
//     w: u32,
//     h: u32,
//     x: u32,
//     y: u32,
// }

// #[derive(Deserialize, Serialize)]
// struct SpriteMap {
//     pub abilities: HashMap<String, SpriteInner>,
//     pub champions: HashMap<String, SpriteInner>,
//     pub items: HashMap<u32, SpriteInner>,
// }

// pub fn generate_sprite_map(out_dir: &str) {
//     let sprite_map_out_path = Path::new(&out_dir).join("sprite_map.br");
//     let location = Path::new("sprite/sprite.json");
//     let map =
//         serde_json::from_str::<SpriteMap>(std::fs::read_to_string(&location).unwrap().as_str())
//             .unwrap();
//     let bytes = compress_bytes!(map);
//     fs::write(sprite_map_out_path, bytes).unwrap();
// }

pub fn generate_sprite_map(_out_dir: &str) {}
