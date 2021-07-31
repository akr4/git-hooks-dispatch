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
    run(".", &opt.hook, &opt.args, &executor::ExecutorImpl::new())
}
