use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "main")]
pub struct Opt {
    #[structopt(name = "ARG")]
    pub args: Vec<String>,
}
