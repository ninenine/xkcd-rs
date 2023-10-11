# xkcd-rs 🚀

A Rust utility for downloading and saving XKCD comics.

![XKCD Logo](code_lifespan.png)

## Features

- Download and save XKCD comics directly from the official XKCD API.
- Concurrent downloads for faster fetching.
- Resilient to network issues with retry mechanisms.

## Requirements

- Rust 2018 Edition or later. You can download Rust from the official website or using Rustup:

  - [Download Rust](https://www.rust-lang.org/tools/install)
  - [Rustup](https://rustup.rs/)

## Installation

Clone the repository:

```bash
git clone https://github.com/ninenine/xkcd-rs.git
cd xkcd-rs
```

Build the project:

```bash
cargo build --release
```

## Usage

After building the project, run:

```bash
./target/release/xkcd-rs
```

This will start downloading all XKCD comics to the specified directory.

## Configuration

You can modify certain parameters, such as the maximum number of concurrent downloads, by editing the source. Future versions may support a configuration file or command-line arguments.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Inspired by the wonderful world of [XKCD](https://xkcd.com/).
- Thanks to all contributors and the Rust community.
