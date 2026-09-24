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

    /// Scale factor for the sprite (eg. 0.5 is half size, 1.0 is full size).
    /// Values below 1.0 lose pixels, since each terminal cell can only hold
    /// two colors.
    #[arg(short, long, default_value_t = 1.0)]
    pub scale: f32,
}