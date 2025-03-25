use ::clap::Parser;
use std::{io, fs, env};
use std::path::Path;
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
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let content = fs::read_to_string(&path)?;
        toml::from_str::<toml::Value>(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
    };

    let tasks = cargo_toml
        .get("package")
        .and_then(|x| x.get("metadata"))
        .and_then(|x| x.get("tasks"))
        .and_then(|x| x.as_table())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "`package.metadata.tasks` table not found"))?;

    let task = tasks
        .get(&task_name)
        .and_then(|x| x.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("task `{task_name}` not found")))?;

    let status = Command::new(option_env!("SHELL").unwrap_or("sh"))
        .args(["-c", task])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    let code = status
        .code()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "process terminated by signal"))?;

    exit(code)
}
