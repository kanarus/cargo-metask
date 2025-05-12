use ::clap::Parser;
use ::std::{io, fs, env};
use ::std::process::{Command, Stdio, exit};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    task_names: Vec<String>,
}

pub fn run() -> io::Result<()> {
    let Args {
        task_names,
    } = {
        let mut args = env::args().collect::<Vec<_>>();
        if matches!(args.get(1).map(String::as_str), Some("task" | "metask")) {
            // when invoked as `cargo task` or `cargo metask`
            args.remove(1);
        }
        Args::parse_from(args)
    };
    if task_names.is_empty() {
        eprintln!("[cargo-metask] no task names provided.");
        return Ok(());
    }

    let cargo_toml = {
        toml::from_str::<toml::Value>(&fs::read_to_string("Cargo.toml")?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
    };

    let task_def = get_task_def(&cargo_toml).ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidData,
        "`{package, workspace}.metadata.tasks` not found"
    ))?;

    let tasks = {
        let mut tasks = Vec::with_capacity(task_names.len());
        for task_name in &task_names {
            let task = task_def
                .get(task_name)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("task `{task_name}` not found")))?
                .as_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("task `{task_name}` is not a string")))?;
            tasks.push(task);
        }
        tasks
    };

    // execute tasks in parallel...

    let shell = env::var("SHELL");
    let shell = shell.as_deref().unwrap_or("/bin/sh");

    let mut handles = std::collections::VecDeque::with_capacity(tasks.len());
    for task in &tasks {
        let task = format!("set -Cue\n{task}");
        handles.push_back(
            Command::new(shell)
                .args(["-c", &task])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
        );
    }

    let mut error_code = None;
    while let Some(mut next) = handles.pop_front() {
        match next.try_wait()? {
            // task is still running, so push it back to the queue
            None => handles.push_back(next),

            // task has finished, so check its exit status
            Some(code) => match code.code() {
                Some(code) => {
                    if code != 0 && error_code.is_none() {
                        error_code = Some(code);
                    }
                }
                None => {
                    eprintln!("[cargo-metask] task terminated by signal");
                    if error_code.is_none() {
                        error_code = Some(1);
                    }
                }
            }
        }

        // Sleep for a short time to avoid busy waiting.
        // 
        // This will not be a problem in practice :
        // * the tasks are usually short-lived
        // * the queue is small
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    exit(error_code.unwrap_or(0));
}

fn get_task_def(cargo_toml: &toml::Value) -> Option<&toml::Table> {
    (cargo_toml.get("workspace")).or(cargo_toml.get("package"))?
        .get("metadata")?
        .get("tasks")?
        .as_table()
}
