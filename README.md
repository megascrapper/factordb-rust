# factordb-rust

Rust wrapper for [FactorDB](http://factordb.com/) API.

Includes a library as well as a simple command line app.

## Command line app

### Installation

```
cargo install factordb --all-features
```

## Building from source

```
git clone https://github.com/megascrapper/factordb-rust
cd factordb-rust
cargo build
```

### Command line usage

```
Rust wrapper for FactorDB API

Usage: factordb [OPTIONS] <NUMBER>

Arguments:
  <NUMBER>  Number to find its factor

Options:
      --unique   Print unique factors on each line
      --json     Print JSON output of FactorDB API
  -h, --help     Print help
  -V, --version  Print version
```

## Library

### Add dependency

Add the following to your `Cargo.toml`:

```toml
factordb = "0.3.0"
```

### Library usage example

```rust
use std::error::Error;
use factordb::FactorDbClient;
use num_bigint::BigInt; // All numeric values in the result object are of this type

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialise the client
    let client = FactorDbClient::new();

    // Make requests
    let forty_two = client.get(42).await?;
    let expect_factors: Vec<BigInt> = vec![2, 3, 7].into_iter().map(|n| BigInt::from(n)).collect();
    assert_eq!(forty_two.into_factors_flattened(), expect_factors);

    Ok(())
 }
```

### Documentation

<https://docs.rs/factordb/0.3.0>

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
