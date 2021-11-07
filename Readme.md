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

## Notes

Recently I looked into how to use Rust to write iOS apps. Most articles describe the hybrid approach of using Rust via FFI, where you use Rust for the core logic and Swift/ObjC/Flutter for the UI. Using 2 languages + FFI can be clumsy and error prone, so I was looking for writing the whole app in Rust.

A few things I learned along the way:
- The closet thing I found is [cacao](https://github.com/ryanmcgrath/cacao), which has a very preliminary iOS support. Proved defintely we can write iOS apps with Rust only though.
- Based on `cacao` code, I created [objc-derive](https://github.com/wooden-worm/objc-derive) to help with exposing UIKit to Rust. This repo is an example of interacting with UIKit via `objc-derive`.
- Although it works, UIKit in Rust is super verbose.
- [iced](https://github.com/iced-rs/iced) runs, but kinda buggy with iOS. I saw some intermittent crashes in real devices. It looks like something 
- `iced` + `objc-derive` is an interesting combo. You can write the UI with `iced` and interact with iOS with `objc-derive`