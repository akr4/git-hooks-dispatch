use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-hooks-dispatch")]
pub struct Opt {
    #[structopt(name = "hook")]
    pub hook: String,
    #[structopt(name = "hooks-dir", short, long)]
    pub hooks_dir: Option<String>,
    #[structopt(short, long)]
    pub verbose: bool,
    #[structopt(name = "arg")]
    pub args: Vec<String>,
}
