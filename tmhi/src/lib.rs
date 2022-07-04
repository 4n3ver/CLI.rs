use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

pub mod nokia;
pub mod result;

#[derive(StructOpt, Debug)]
pub struct CliArgs {
    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
}

pub fn run(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    Ok(())
}
