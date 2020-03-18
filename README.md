<p align="center">
  <a href="https://emojipedia.org/twitter/twemoji-12.1.4/bird/">
    <img alt="Bird on Twitter Twemoji 12.1.4" src="lory.png" width="72">
  </a>
  <p align="center">
    <i>
    lory is an easy-to-use Rust library for accessing the Twitter API.
    </i>
  </p>
</p>

<p align="center">
  <a href="https://crates.io/crates/lory"><img alt="lory crates" src="https://img.shields.io/crates/v/lory.svg?color=%23fdc452"></a>
  <a href="https://docs.rs/lory"><img alt="lory docs" src="https://docs.rs/lory/badge.svg?color=%233b6837"></a>
  <a href="https://github.com/pyk/lory/actions"><img alt="lory" src="https://github.com/pyk/lory/workflows/lory/badge.svg?branch=master"></a>
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

