use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::types::Result;

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
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(CommandError {
            program: spec.program.to_string(),
            code: output.status.code(),
        }
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

pub struct CommandSpec<'a> {
    program: &'a str,
    args: Vec<OsString>,
    cwd: Option<&'a Path>,
    envs: Vec<(&'a str, OsString)>,
    env_removes: Vec<&'a str>,
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

    pub fn env(mut self, key: &'a str, val: impl Into<OsString>) -> Self {
        self.envs.push((key, val.into()));
        self
    }

    pub fn env_remove(mut self, key: &'a str) -> Self {
        self.env_removes.push(key);
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
            Some(code) => write!(f, "command `{}` failed with exit code {}", self.program, code),
            None => write!(f, "command `{}` terminated by signal", self.program),
        }
    }
}

impl std::error::Error for CommandError {}
