use clap:: {
    Args,
    Parser,
    Subcommand
};
#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct  Arg{
    pub path: String,
}