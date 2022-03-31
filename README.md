# factordb-rust

Rust wrapper for [FactorDB](http://factordb.com/) API.

Includes a library as well as a simple command line app.

## Installation
## Cargo
```
cargo install factordb
```

## Building from source
```
git clone https://github.com/megascrapper/factordb-rust
cd factordb-rust
cargo build
```

## Usage
```
factordb <NUMBER>
```

### Options
```
    -h, --help             Print help information
        --print-factors    Print all factors (including repeating ones) on each line
    -V, --version          Print version information

```

## Feature wishlist
- [ ] Async mode
- [ ] More testing
- [ ] get the queried number as BigInt
- [x] A method to get unique factors
- [x] Have the internal representation be in native bigint instead of a String (may require breaking change).

## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.