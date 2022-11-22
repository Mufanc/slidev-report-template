use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    #[clap(short, long, value_names = &["ADDR"])]
    pub bind: Option<String>,

    #[clap(short, long = "root-dir", value_names = &["DIR"], default_value="dist")]
    pub root_dir: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
