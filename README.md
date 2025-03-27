# cargo-metask

A lightweight task runner for tasks defined in Cargo.toml

## Installation

```sh
cargo install cargo-metask
```

Then `cargo metask` and alias `cargo task` will be available.

## Usage

1. Define tasks in `package.metadata.tasks` or `workspace.metadata.tasks` of your `Cargo.toml` :

```toml
[package.metadata.tasks]
greet = "echo 'Hello, metask!'"
```

2. Run a task :

```sh
cargo metask greet

# or

cargo task greet
```

## Development

cargo-metask is currently in early development stage and may have some unexpected behavior or missing features. If you have any feedback or suggestions, feel free to open [Issues](https://github.com/kanarus/cargo-metask/issues) or [Pull requests](https://github.com/kanarus/cargo-metask/pulls)!

## License

cargo-metask is licensed under [MIT LICENSE](LICENSE).
