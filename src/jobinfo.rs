use std::process::Command;

use clap::Args;
use console::style;
use eyre::{eyre, Result, Context};
use log::debug;
use tabled::{
    object::{Columns, Rows},
    width::{Max, MinWidth},
    Disable, Format, Modify, Width,
};
use terminal_size::terminal_size;


#[derive(Args)]
pub struct JobInfo {
    #[clap(short, long, value_parser)]
    pub jobid: u32,
}

pub fn jobinfo(jobid: u32) -> Result<()> {
    debug!("jobid = {}", jobid);

    let output = Command::new("sacct")
        .arg("-plj")
        .arg(format!("{}", jobid))
        .arg("--delimiter='|'")
        .output().context("Failed to run 'sacct' command")?;
    if !output.status.success() {
        return Err(eyre!("Command failed!"));
    }

    let stdout = String::from_utf8(output.stdout)?;

    let mut table_builder = tabled::builder::Builder::default();
    for row in stdout
        .lines()
            .enumerate()
            .filter(|&(ii, _)| ii != 1)
            .map(|(_, v)| v)
            {
                // TODO: There must be a better way to do this than creating a vec and popping! (peekable?)
                let mut cols: Vec<_> = row.split('|').collect();
                cols.pop();
                table_builder.add_record(cols);
            }
    let mut table_builder = table_builder.index();
    table_builder.transpose().hide_index();
    let table = table_builder.build();
    let width: usize = terminal_size()
        .ok_or(eyre!("Failed to get current terminal width"))?
        .0
        .0
        .into();
    debug!("Term width = {width}");

    let table = table
        .with(Disable::Row(0..1))
        .with(tabled::style::Style::psql())
        .with(Modify::new(Rows::first()).with(MinWidth::new(Max)))
        .with(Width::wrap(width).keep_words())
        .with(Modify::new(Columns::single(0)).with(Format::new(|s| style(s).yellow().to_string())));

    println!("{table}");

    Ok(())
}

