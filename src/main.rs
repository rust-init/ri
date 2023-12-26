
mod args;
use args::Arg;
use std::process::ExitCode;
use clap::Parser;


fn main() -> ExitCode{
    let args = Arg::parse();
    let codes = std::fs::read_to_string(args.path).unwrap();
    
    ExitCode::SUCCESS
}