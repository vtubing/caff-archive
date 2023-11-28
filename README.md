# caff-archive [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/caff-archive.svg
[crates.io]: https://crates.io/crates/caff-archive

## What is it?

A library for working with binary CAFF archives. This is the format used for `.cmo3` and `.can3` files.

## How do I obtain this majestic tool?

Run the following Cargo command in your project directory (assuming you have [cargo-edit](https://github.com/killercup/cargo-edit) installed):

```fish
cargo add caff-archive
```

Or add the following line to your `Cargo.toml` (in the `[dependencies]` array):

```toml
caff-archive = "^ 0.1"
```

## How do I use it?

```rust
fn main() {
  let mut file = File::open(&archive).expect("failed to open archive for reading");
  let mut archive = caff_archive::Archive::read(&mut file).expect("failed to read archive from input data");
}
```

## Optional Features

- `logging` enables tracing and debugging logs when reading and writing archives.
- `discovery` implies `logging` and enables additional logging of potentially
  interesting scenarios, from  the perspective of this library. It will report
  things like raw values that were parsed into fallback values in enums, or data
  sections that are expected to be padding but contain non-zero bytes, along
  with different potential ways those bytes could be parsed, among other things.

## How was this made?

- Carefully, without using or referencing any code or libraries from the format vendor.
- The [ImHex](https://github.com/WerWolv/ImHex) highlighting patterns from the [MOC3ingbird Exploit](https://github.com/OpenL2D/moc3ingbird) (CVE-2023-27566) was instrumental in understanding this format.
- The discovery process for undocumented binary formats is described [here](https://gist.github.com/colstrom/f671d1583662de47b505a42a75b3a44b).

## License

`caff-archive` is available under the MIT License. See `LICENSE.txt` for the full text.

While the license is short, it's still written in fancy lawyer-speak. If you
prefer more down-to-earth language, consider the following:

- tl;drLegal has a simple visual summary available [here](https://www.tldrlegal.com/license/mit-license).
- FOSSA has a more in-depth overview available [here](https://fossa.com/blog/open-source-licenses-101-mit-license/).
