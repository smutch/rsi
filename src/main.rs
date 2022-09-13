use std::process::Command;

use clap::Parser;
use log::debug;
use eyre::{Result, eyre};

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    jobid: u32
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    debug!("jobid = {}", args.jobid);

    let output = Command::new("cat").arg("tmp.txt").output()?;
    if !output.status.success() {
        return Err(eyre!("Command failed!"));
    }

    let stdout = String::from_utf8(output.stdout)?;

    // htable = horizontal table
    let htable: Vec<_> = stdout.lines().collect();
    let mut rows: Vec<_> = htable[0].split('|').map(|v| vec!(v)).collect();

    for entry in &htable[2..] {
        entry.split('|').zip(&mut rows).for_each(|(e, r)| r.push(e));
    }

    for row in rows {
        println!("{}", row.join(" | "));
    }

    Ok(())
}
