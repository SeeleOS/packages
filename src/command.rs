use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::types::Result;

pub fn make<'a>() -> CommandSpec<'a> {
    let jobs = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    CommandSpec::new("make").arg(format!("-j{jobs}"))
}

pub fn run(spec: CommandSpec<'_>) -> Result<()> {
    let mut cmd = Command::new(spec.program);
    cmd.args(spec.args);
    if let Some(cwd) = spec.cwd {
        cmd.current_dir(cwd);
    }
    for (key, val) in spec.envs {
        cmd.env(key, val);
    }
    for key in spec.env_removes {
        cmd.env_remove(key);
    }
    if let Some(path) = spec.stdin_file {
        let file = fs::File::open(path)?;
        cmd.stdin(Stdio::from(file));
    }
    let status = cmd.status()?;
    if !status.success() {
        return Err(CommandError {
            program: spec.program.to_string(),
            code: status.code(),
        }
        .into());
    }
    Ok(())
}

pub fn capture(spec: CommandSpec<'_>) -> Result<String> {
    let output = run_output(spec)?;
    if !output.status.success() {
        return Err(CommandError {
            program: "<captured command>".to_string(),
            code: output.status.code(),
        }
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

pub fn run_output(spec: CommandSpec<'_>) -> Result<std::process::Output> {
    let mut cmd = Command::new(spec.program);
    cmd.args(spec.args);
    if let Some(cwd) = spec.cwd {
        cmd.current_dir(cwd);
    }
    for (key, val) in spec.envs {
        cmd.env(key, val);
    }
    for key in spec.env_removes {
        cmd.env_remove(key);
    }
    if let Some(path) = spec.stdin_file {
        let file = fs::File::open(path)?;
        cmd.stdin(Stdio::from(file));
    }
    Ok(cmd.output()?)
}

pub struct CommandSpec<'a> {
    program: &'a str,
    args: Vec<OsString>,
    cwd: Option<&'a Path>,
    envs: Vec<(String, OsString)>,
    env_removes: Vec<String>,
    stdin_file: Option<&'a Path>,
}

impl<'a> CommandSpec<'a> {
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            args: Vec::new(),
            cwd: None,
            envs: Vec::new(),
            env_removes: Vec::new(),
            stdin_file: None,
        }
    }

    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    pub fn cwd(mut self, cwd: &'a Path) -> Self {
        self.cwd = Some(cwd);
        self
    }

    pub fn env(mut self, key: impl Into<String>, val: impl Into<OsString>) -> Self {
        self.envs.push((key.into(), val.into()));
        self
    }

    pub fn env_append(
        mut self,
        key: impl Into<String>,
        val: impl Into<OsString>,
        separator: &str,
    ) -> Self {
        let key = key.into();
        let val = val.into();
        if let Some((_, existing)) = self.envs.iter_mut().rev().find(|(k, _)| *k == key) {
            if !existing.is_empty() && !val.is_empty() {
                let mut merged = existing.clone();
                merged.push(separator);
                merged.push(&val);
                *existing = merged;
            } else if !val.is_empty() {
                *existing = val;
            }
        } else {
            self.envs.push((key, val));
        }
        self
    }

    pub fn env_remove(mut self, key: impl Into<String>) -> Self {
        self.env_removes.push(key.into());
        self
    }

    pub fn stdin_file(mut self, path: &'a Path) -> Self {
        self.stdin_file = Some(path);
        self
    }
}

#[derive(Debug)]
struct CommandError {
    program: String,
    code: Option<i32>,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(
                f,
                "command `{}` failed with exit code {}",
                self.program, code
            ),
            None => write!(f, "command `{}` terminated by signal", self.program),
        }
    }
}

impl std::error::Error for CommandError {}
