# Rust Columns SEGA 1990 game for the terminal

`rust_columns` is a fun and easy to play game for the terminal, written in Rust. It is a work in progress game, that I use to learn several concepts of the Rust programming language, while doing so on a fun project.

### Implemented behaviors

- [x] Matching blocks on all cardinal axes
- [x] Scoring points
- [x] Losing game

### Planned improvements

- [ ] Ranking
- [ ] Menu
- [ ] Speed optimizations for falling blocks
- [ ] Levels
- [ ] Sounds
- [ ] Resizing (2x)

### Compile the game

You can run compile and run the game simply with:

```shell
$ cargo run
```

### Using the binaries

If you wish to just play the game without compiling, head to the release page and find [artifacts for multiple targets](https://github.com/Rendez/rust_columns/releases).

[![asciicast](https://asciinema.org/a/SaKyJdXfD3jKZh67SBC4mrrAe.svg)](https://asciinema.org/a/SaKyJdXfD3jKZh67SBC4mrrAe)

### Working terminals

Under the hood, all this library is using is the crate `crossterm`, therefore the same support applies:

- Windows Powershell
  - Windows 10 (Pro)
- Windows CMD
  - Windows 10 (Pro)
  - Windows 8.1 (N)
- Ubuntu Desktop Terminal
  - Ubuntu 17.10
- (Arch, Manjaro) KDE Konsole
- Linux Mint
- Terminal.app
- Alacritty
- iTerm and more!

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).
