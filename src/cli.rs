use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The trainer to display, use "random" to get a random trainer.
    pub trainer: Vec<String>,

    /// Whether to hide the trainer's name which appears above the sprite.
    #[arg(long, default_value_t = false)]
    pub hide_name: bool,

    /// List all available trainers and exit.
    #[arg(long, default_value_t = false)]
    pub list: bool,
}