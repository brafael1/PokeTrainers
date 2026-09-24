# poketrainers

Display Pokemon trainer sprites in your terminal.

Inspired by (and adapted from) [pokeget-rs](https://github.com/talwat/pokeget-rs),
but for the **1,500 trainer sprites** used by [Pokemon Showdown](https://play.pokemonshowdown.com/sprites/trainers/)
instead of Pokemon sprites.

## Usage

```sh
poketrainers <trainer> [<trainer>...]
poketrainers random
poketrainers --list
```

Examples:

```sh
poketrainers red                    # just Red
poketrainers red ash lance cynthia  # several at once
poketrainers allister-unmasked      # variant names work too
poketrainers --hide-name blue       # omit the name printed above
poketrainers random                 # a random trainer
poketrainers --list | grep gen1     # find all available names
```

The name is printed to stderr and the sprite to stdout, so you can save just the
sprite with `poketrainers red > red.txt`.

If you typo a name, poketrainers suggests close matches:

```sh
$ poketrainers ashh
trainer not found: ashh
did you mean: ash
```

## How it works

- All 1,500 sprites are compiled into the binary with `rust-embed`, so it runs
  offline and fast — no downloads at runtime.
- The list of trainers is derived directly from the embedded file names, so the
  CLI always matches the sprites (no manifest to keep in sync).
- Sprites are drawn 2x smaller using the half-block trick (`▀`/`▄`) with
  truecolor ANSI escapes, via the `showie` crate.

## Requirements

- A terminal with truecolor (24-bit) support.

## Installation

```sh
cargo install --path .
```

## Credits & licensing

The trainer sprites belong to Pokemon Showdown and their respective artists —
**see [CREDITS.md](CREDITS.md) before using them in a public project.**
The Rust code in this repository is MIT licensed (see `LICENSE`).