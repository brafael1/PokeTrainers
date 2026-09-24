use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "The trainer to display, use \"random\" to get a random trainer.")]
    pub trainer: Vec<String>,

    #[arg(
        long,
        default_value_t = false,
        help = "Whether to hide the trainer's name which appears above the sprite."
    )]
    pub hide_name: bool,

    #[arg(long, default_value_t = false, help = "List all available trainers and exit.")]
    pub list: bool,

    #[arg(
        short,
        long,
        default_value_t = 1.0,
        help = "Scale factor for the sprite (eg. 0.5 is half size, 1.0 is full size). Values below 1.0 lose pixels."
    )]
    pub scale: f32,
}