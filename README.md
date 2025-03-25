# cargo-metask

A lightweight task runner for tasks defined in Cargo.toml

## Installation

```sh
cargo install cargo-metask
```

Then `cargo metask` and alias `cargo meta` will be available.

## Usage

1. Define tasks in `packeage.metadata.tasks` of your `Cargo.toml` :

```toml
[package.metadata.tasks]
greet = "echo 'Hello, metask!'"
```

2. Run a task :

```sh
cargo metask greet

# or

cargo meta greet
```

## License

cargo-metask is licensed under [MIT LICENSE](LICENSE).
