use clap::{Parser, Subcommand};
use eyre::Result;

mod jobinfo;
mod starttime;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
/// Top-level command line arguments for `rsi`
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
/// Register subcommands (currently just `jobinfo`)
enum Commands {
    #[clap(name = "jobinfo")]
    JobInfo(jobinfo::JobInfo),
    #[clap(name = "starttime")]
    StartTime(starttime::StartTime),
}

fn main() -> Result<()> {
    env_logger::init();

    let cmd = Cli::parse();

    match &cmd.command {
        Commands::JobInfo(args) => jobinfo::jobinfo(args.jobid, &args.step),
        Commands::StartTime(args) => starttime::starttime(&args.script),
    }
}
