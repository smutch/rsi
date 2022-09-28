// TODO: Put the JobName as the header

use std::process::Command;

use clap::Args;
use console::style;
use eyre::{eyre, Context, Result};
use log::debug;
use tabled::{
    object::{Columns, Rows, Cell},
    width::{Max, Min, MinWidth, PriorityMax},
    Disable, Format, Modify, Width,
};
use terminal_size::terminal_size;

#[derive(Args)]
/// Command line arguments for the jobinfo subcommand
pub struct JobInfo {
    #[clap(value_parser)]
    pub jobid: u32,
    #[clap(short, long, default_value="0")]
    pub step: String,
}

/// Print the output from `sacct -pj <jobid>` **for all available columns** in a nice (actually readable!) format.
///
/// # Arguments
///
/// * `jobid` - The SLURM jobid to request info for from `saccct`
/// * `step` - The step to request info for from `saccct` (e.g. "batch", "0", ...)
pub fn jobinfo(jobid: u32, step: &str) -> Result<()> {
    let full_jobid = format!("{jobid}.{step}");
    debug!("full_jobid = {}", full_jobid);

    let output = Command::new("sacct")
        .arg("-e")
        .output()
        .context("Failed to run 'sacct' command")?;
    if !output.status.success() {
        return Err(eyre!("Command failed!"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    debug!("...\n{stdout}");

    let columns = stdout
        .lines()
        .flat_map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .join(",");

    let output = Command::new("sacct")
        .arg("-o")
        .arg(columns)
        .arg("--delimiter=|")
        .arg("-pj")
        .arg(full_jobid)
        .output()
        .context("Failed to run 'sacct' command")?;
    if !output.status.success() {
        return Err(eyre!("Command failed!"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    debug!("...\n{stdout}");

    let mut table_builder = tabled::builder::Builder::default();

    for row in stdout
        .lines()
    {
        // TODO: There must be a better way to do this than creating a vec and popping! (peekable?)
        let mut cols: Vec<_> = row.split('|').collect();
        cols.pop();
        debug!("{cols:?}");
        table_builder.add_record(cols);
    }

    let mut table_builder = table_builder.index();
    table_builder.transpose();
    table_builder.set_index(0).set_name(None);
    let table = table_builder.build();
    let width: usize = terminal_size()
        .ok_or(eyre!("Failed to get current terminal width"))?
        .0
         .0
        .into();
    debug!("Term width = {width}");

    let table = table
        .with(tabled::style::Style::psql())
        .with(Modify::new(Cell(0, 1)).with(|_: &str| format!("JOBID={jobid} STEP={step}")))
        .with(Modify::new(Columns::new(1..)).with(Width::wrap(width-25).keep_words()))
        .with(
            Modify::new(Columns::single(0))
                .with(Format::new(|s| style(s).yellow().to_string()))
        );

    println!("{table}");

    Ok(())
}
