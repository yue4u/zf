# Project: ZF

<p align="center"><img width="500" height="500" src="./zf/assets/ZF.png" /></p>

## About

This is an experimental game about using command line to accomplish tasks and fight in space.

This game is made with [Godot Engine](https://godotengine.org/) and written in [Rust](https://www.rust-lang.org/) using [GDNative](https://github.com/godot-rust/gdnative).

This project internally use a compiled [WASI (The WebAssembly System Interface)](https://wasi.dev/) version of [nushell](https://www.nushell.sh/) for command parsing / executing while using [Wasmtime](https://wasmtime.dev/) as the runtime at the same time. The in-game terminal emulator is powered by [wezterm](https://wezfurlong.org/wezterm/) and rendering is handled in native godot APIs.

This game is mainly developed on Linux but also available for macOS and Windows.

## Documents

- [Screenshots](./docs/screenshots.md)
- [Architecture](./docs/architecture.md)

## License & Credits

License: [MIT](./LICENSE) and avalible on [Github](https://github.com/yue4u/zf). For libraries and other assets, see [License & Credits](./CREDITS.md) and [LICENSE-THIRD-PARTY](./docs/LICENSE-THIRD-PARTY).