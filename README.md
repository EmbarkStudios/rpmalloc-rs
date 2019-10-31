# 🛄 rpmalloc-rs

[![Build Status](https://github.com/EmbarkStudios/rpmalloc-rs/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/rpmalloc-rs/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/rpmalloc.svg)](https://crates.io/crates/rpmalloc)
[![Docs](https://docs.rs/rpmalloc/badge.svg)](https://docs.rs/rpmalloc)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](http://embark.dev)

WIP implementation of [rpmalloc](https://github.com/rampantpixels/rpmalloc) as a Rust global memory allocator.

See the [rpmalloc README](https://github.com/mjansson/rpmalloc/blob/master/README.md) for a detailed description of how the allocator works, peforms, and compares with other allocators.

## How to use

To use rpmalloc as the global allocator in your Rust binary crate, in `Cargo.toml` add:

```toml
[dependencies]
rpmalloc = "0.1.0"
```

And then in one of your `.rs` files:

```rust
#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;
```

## Contributing

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Note that the [rpmalloc](https://github.com/rampantpixels/rpmalloc) library this crate uses is under public domain, and can also be licensed under MIT.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
