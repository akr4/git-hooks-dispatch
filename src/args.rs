use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-hooks-dispatch")]
pub struct Opt {
    #[structopt(name = "hook")]
    pub hook: String,
    #[structopt(name = "ARG")]
    pub args: Vec<String>,
}
