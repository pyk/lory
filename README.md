<p align="center">
  <img alt="lory Logo" src="lory.png">
  <p align="center" style="font-size:32px;"><b>lory</b></p>
  <p align="center">
    <i>
    lory is an easy-to-use fundamental library for accessing the Twitter API.
    </i>
  </p>
</p>

<p align="center">
  <a href="https://travis-ci.com/pyk/lory"><img alt="lory min rustc" src="https://travis-ci.com/pyk/lory.svg?branch=master"></a>
  <a href="https://crates.io/crates/lory"><img alt="lory crates" src="https://img.shields.io/crates/v/lory.svg?color=%23fdc452"></a>
  <a href="https://docs.rs/lory"><img alt="lory docs" src="https://docs.rs/lory/badge.svg?color=%233b6837"></a>
  <a href="https://crates.io/crates/lory"><img alt="lory min rustc" src="https://img.shields.io/badge/rustc-stable-green.svg"></a>
</p>

## Documentation
- [Quickstart Tutorial][quickstart tutorial]
- [API Reference]

[API Reference]: https://docs.rs/lory

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
lory = "*"
```

and this to your crate root:

```rust
use lory::prelude::*;
```

To get started using lory, read the [quickstart tutorial].

[quickstart tutorial]:  https://docs.rs/lory#quickstart-tutorial

## Development
Clone the repository using the following command:

```sh
git clone https://github.com/pyk/lory.git --depth=1
cd lory/
```

To build the project, use the following command:

```sh
cargo build
```

To run the tests, use the following command:

```sh
cargo test
```

## Getting help
Feel free to start discussion at [GitHub issues].

[Github issues]: https://github.com/pyk/lory/issues/new/choose

## License
lory is licensed under the [Apache-2.0](./LICENSE) license.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in lory by you, as defined in the Apache-2.0
license, shall be licensed as above, without
any additional terms or conditions.

