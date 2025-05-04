<div align="center">
    <h1>
        cargo-metask
    </h1>
    <p>
        Cargo task runner for <code>{package, workspace}.metadata.tasks</code>
    </p>
</div>

<div width="100%" align="right">
    <a href="https://github.com/kanarus/cargo-metask/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/crates/l/cargo-metask.svg" /></a>
    <a href="https://github.com/kanarus/cargo-metask/actions"><img alt="CI status" src="https://github.com/kanarus/cargo-metask/actions/workflows/CI.yml/badge.svg"/></a>
    <a href="https://crates.io/crates/cargo-metask"><img alt="crates.io" src="https://img.shields.io/crates/v/cargo-metask" /></a>
</div>

## Installation

```sh
cargo install cargo-metask
```

Then `cargo metask` and alias `cargo task` will be available.

## Usage

1. Define tasks in `package.metadata.tasks` or `workspace.metadata.tasks` table of your `Cargo.toml` :

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

## Parallel Execution

When multiple task names are given :

```sh
cargo task task-a task-b task-c
```

the tasks are executed **in parallel**.

## License

cargo-metask is licensed under [MIT LICENSE](LICENSE).
