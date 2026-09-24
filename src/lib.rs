use rust_embed::RustEmbed;

pub mod cli;
pub mod list;
pub mod sprites;
pub mod trainer;

#[derive(RustEmbed)]
#[folder = "data/trainers/"]
pub struct Data;