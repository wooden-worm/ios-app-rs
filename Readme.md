Proof of concept: iOS app written in pure Rust

## To Run
- Install [cargo bundle](https://github.com/burtonageo/cargo-bundle)
- Install [cargo make](https://github.com/sagiegurari/cargo-make)
- Start an iOS simulator
- `cargo make run_ios_sim`

The heavy lifting is done by [objc-derive](https://github.com/wooden-worm/objc-derive).

`objc-derive` is a proc macro crate to help:
- Exposing `objc` objects/methods to Rust
- Implementing `objc`'s `selector` in pure Rust