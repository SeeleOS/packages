use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::trace::{command, command_detail};
use crate::types::Result;

pub fn run(spec: CommandSpec<'_>) -> Result<()> {
    log_command("run", &spec);
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
    command(format!(
        "command finished: program=`{}` status={}",
        spec.program, status
    ));
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
    log_command("capture", &spec);
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
    command(format!(
        "captured command finished: program=`{}` status={} stdout_bytes={} stderr_bytes={}",
        spec.program,
        output.status,
        output.stdout.len(),
        output.stderr.len()
    ));
    if !output.status.success() {
        return Err(CommandError {
            program: spec.program.to_string(),
            code: output.status.code(),
        }
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn log_command(mode: &str, spec: &CommandSpec<'_>) {
    command(format!("{mode} {}", spec.describe()));
    if let Some(cwd) = spec.cwd {
        command_detail(format!("cwd={}", cwd.display()));
    }
    if !spec.envs.is_empty() {
        let envs = spec
            .envs
            .iter()
            .map(|(key, value)| format!("{key}={}", value.to_string_lossy()))
            .collect::<Vec<_>>()
            .join(", ");
        command_detail(format!("env overrides: {envs}"));
    }
    if !spec.env_removes.is_empty() {
        command_detail(format!("env removed: {}", spec.env_removes.join(", ")));
    }
    if let Some(path) = spec.stdin_file {
        command_detail(format!("stdin redirected from {}", path.display()));
    }
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

    fn describe(&self) -> String {
        let mut parts = vec![self.program.to_string()];
        parts.extend(self.args.iter().map(|arg| shell_escape(arg)));
        parts.join(" ")
    }
}

fn shell_escape(value: &OsStr) -> String {
    let text = value.to_string_lossy();
    if text.is_empty() {
        "''".to_string()
    } else if text
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '_' | '-' | '=' | ':'))
    {
        text.into_owned()
    } else {
        format!("{text:?}")
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
