//! Display pokemon trainer sprites in your terminal.

use clap::Parser;
use poketrainers::cli::Args;
use poketrainers::list::List;
use poketrainers::sprites;
use poketrainers::trainer::Trainer;
use std::process::exit;

fn main() {
    let args = Args::parse();

    if args.list {
        let list = List::read();
        for name in list.iter() {
            println!("{}", name);
        }
        return;
    }

    if args.trainer.is_empty() {
        eprintln!("you must specify the trainer you want to display");
        eprintln!("run `poketrainers --list` to see all available trainers");
        exit(1);
    }

    let list = List::read();
    let trainers: Vec<Trainer> = args
        .trainer
        .into_iter()
        .map(|arg| Trainer::new(arg, &list, args.scale))
        .collect();

    let combined = sprites::combine(&trainers);
    if !args.hide_name {
        let names: Vec<&str> = trainers.iter().map(|x| x.name.as_str()).collect();
        eprintln!("{}", names.join(", "));
    }

    println!("{}", showie::render(&combined));
}