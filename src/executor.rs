use anyhow::Result;
#[cfg(test)]
use mockall::automock;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[cfg_attr(test, automock)]
pub trait Executor {
    fn execute(&self, path: &Path, args: &Vec<String>) -> Result<i32>;
}

pub struct ExecutorImpl {}

impl ExecutorImpl {
    pub fn new() -> Self {
        ExecutorImpl {}
    }
}

impl Executor for ExecutorImpl {
    fn execute(&self, path: &Path, args: &Vec<String>) -> Result<i32> {
        let output = Command::new(path).args(args).output()?;
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();
        Ok(output.status.code().unwrap_or(1))
    }
}
