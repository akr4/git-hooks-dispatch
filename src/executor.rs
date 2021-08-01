use anyhow::Result;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Executor {
    fn execute(&self, working_dir: &Path, hook_path: &Path, args: &Vec<String>) -> Result<i32>;
}

pub struct ExecutorImpl {}

impl ExecutorImpl {
    pub fn new() -> Self {
        ExecutorImpl {}
    }
}

impl Executor for ExecutorImpl {
    fn execute(&self, working_dir: &Path, hook_path: &Path, args: &Vec<String>) -> Result<i32> {
        let output = Command::new(hook_path)
            .args(args)
            .current_dir(working_dir)
            .output()?;
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
        Ok(output.status.code().unwrap_or(1))
    }
}
