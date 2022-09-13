use std::process::Command;

use clap::Parser;
use eyre::{eyre, Result};
use log::debug;
use tabled::{
    object::{Rows, Columns},
    width::{Max, MinWidth},
    Disable, Modify, Width, Format,
};
use terminal_size::terminal_size;
use console::style;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    jobid: u32,
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
