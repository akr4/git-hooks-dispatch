use crate::args::Opt;
use structopt::StructOpt;

mod args;

fn main() {
    let envvars: Vec<(String, String)> = std::env::vars().collect();
    let envvars: Vec<(String, String)> = envvars.into_iter().filter(|(k, _)| k.starts_with("GIT_")).collect();
    eprintln!("env =============================");
    eprintln!("{:#?}", envvars);

    let opt = Opt::from_args();
    eprintln!("args ============================");
    eprintln!("{:#?}", opt);
}
