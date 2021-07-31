use crate::args::Opt;
use crate::run::run;
use anyhow::Result;
use structopt::StructOpt;

mod args;
mod executor;
mod hook;
mod run;

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();
    let hooks_dir_names = if let Some(hooks_dir) = opt.hooks_dir {
        vec![hooks_dir]
    } else {
        vec!["git-hooks".to_string(), ".git-hooks".to_string()]
    };
    run(
        std::env::current_dir()?,
        &opt.hook,
        &opt.args,
        &executor::ExecutorImpl::new(),
        hooks_dir_names,
    )
}
