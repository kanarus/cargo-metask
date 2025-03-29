use ::clap::Parser;
use std::{io, fs, env};
use std::process::{Command, Stdio, exit};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    task_name: String,
}

pub fn run() -> io::Result<()> {
    let Args {
        task_name,
    } = {
        let mut args = env::args().collect::<Vec<_>>();
        if matches!(args.get(1).map(String::as_str), Some("task" | "metask")) {
            // when invoked as `cargo task` or `cargo metask`
            args.remove(1);
        }
        Args::parse_from(args)
    };

    let cargo_toml = {
        toml::from_str::<toml::Value>(&fs::read_to_string("Cargo.toml")?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
    };

    let tasks = get_tasks(&cargo_toml).ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidData,
        "`{package, workspace}.metadata.tasks` not found"
    ))?;

    let task = tasks
        .get(&task_name)
        .and_then(|x| x.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("task `{task_name}` not found")))?;

    let shell = env::var("SHELL");

    let status = Command::new(shell.as_deref().unwrap_or("sh"))
        .args(["-c", task])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    let code = status
        .code()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "process terminated by signal"))?;

    exit(code)
}

fn get_tasks(cargo_toml: &toml::Value) -> Option<&toml::Table> {
    (cargo_toml.get("workspace")).or(cargo_toml.get("package"))?
        .get("metadata")?
        .get("tasks")?
        .as_table()
}
