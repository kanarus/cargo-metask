use std::{io, fs};
use std::path::Path;
use std::process::{Command, Stdio, exit};

pub fn run() -> io::Result<()> {
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

    let mut args = std::env::args().skip(1);

    let task_name = (match args.next() {
        None => None,
        Some(a) => matches!(&*a, "meta" | "metask")
            .then(|| args.next())
            .unwrap_or(Some(a)),
    }).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "task name not given"))?;

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
