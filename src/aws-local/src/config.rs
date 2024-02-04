use clap::{self, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliConfig {
    #[arg(long)]
    pub function_name: String,

    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,
}
