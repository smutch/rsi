use clap::{Parser, Subcommand};
use eyre::Result;

mod jobinfo;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "jobinfo")]
    JobInfo(jobinfo::JobInfo),
}

fn main() -> Result<()> {
    env_logger::init();

    let cmd = Cli::parse();

    match &cmd.command {
        Commands::JobInfo(args) => jobinfo::jobinfo(args.jobid),
    }
}
