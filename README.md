# cargo-metask

Cargo task runner for `package.metadata.tasks` or `workdspace.metadata.tasks` .

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

## Parallel Execution

When multiple task names are given :

```sh
cargo task task-a task-b task-c
```

the tasks are executed **in parallel**.

## License

cargo-metask is licensed under [MIT LICENSE](LICENSE).
