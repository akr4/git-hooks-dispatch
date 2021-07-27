use crate::args::Opt;
use structopt::StructOpt;

mod args;

fn main() {
    let opt = Opt::from_args();
    eprintln!("args ============================");
    eprintln!("{:#?}", opt);

    let envvars: Vec<(String, String)> = std::env::vars().collect();
    eprintln!("env =============================");
    eprintln!("{:#?}", envvars);
}
