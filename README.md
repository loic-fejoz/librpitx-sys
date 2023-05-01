# librpitx-sys

Rust bindings to [librpitx](https://github.com/F5OEO/librpitx).

## Usage

You should probably not use this crate directly, but use some crates-to-come that will wrap this one in a safer way.

## License

Note that this crate is released under [GNU GENERAL PUBLIC LICENSE](LICENSE) (GPL) as the original librpitx library.
It thus deviates from some habits for Rust crates to make you aware that with default librpitx default installation,
your binary __may__ fall under GPL license anyway.

## How

librpitx-sys contains auto-generated [bindgen](https://rust-lang.github.io/rust-bindgen/) modules.
It only exports `dma` and `gpio` related C++ classes.