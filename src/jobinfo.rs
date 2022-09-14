use std::process::Command;

use clap::Args;
use console::style;
use eyre::{eyre, Context, Result};
use log::debug;
use tabled::{
    object::{Columns, Rows},
    width::{Min, MinWidth},
    Disable, Format, Modify, Width,
};
use terminal_size::terminal_size;

#[derive(Args)]
/// Command line arguments for the jobinfo subcommand
pub struct JobInfo {
    #[clap(short, long, value_parser)]
    pub jobid: u32,
}

/// Print the output from `sacct -pj <jobid>` **for all available columns** in a nice (actually readable!) format.
///
/// # Arguments
///
/// * `jobid` - The SLURM jobid to request info for from `saccct`
pub fn jobinfo(jobid: u32) -> Result<()> {
    debug!("jobid = {}", jobid);

    let output = Command::new("sacct")
        .arg("-e")
        .output()
        .context("Failed to run 'sacct' command")?;
    if !output.status.success() {
        return Err(eyre!("Command failed!"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    debug!("{stdout}");

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
        .arg(format!("{}", jobid))
        .output()
        .context("Failed to run 'sacct' command")?;
    if !output.status.success() {
        return Err(eyre!("Command failed!"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    debug!("{stdout}");

    let mut table_builder = tabled::builder::Builder::default();

    let mut rows = Vec::new();
    for row in stdout
        .lines()
        .enumerate()
        .filter(|&(ii, _)| ii != 1)
        .map(|(_, v)| v)
    {
        // TODO: There must be a better way to do this than creating a vec and popping! (peekable?)
        let mut cols: Vec<_> = row.split('|').collect();
        cols.pop();

        rows.push(cols);
        // table_builder.add_record(cols);
    }

    // find which col stores JobID
    let jobid_col = rows
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|v| v.1 == &"JobID")
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .0;

    // sort by the jobid
    rows[1..].sort_by(|a, b| {
        a.get(jobid_col)
            .unwrap()
            .partial_cmp(b.get(jobid_col).unwrap())
            .unwrap()
    });

    for mut row in rows {
        // make sure that JobID will be the header of the resulting table but all other rows will
        // be in alphabetocal order
        row.swap(0, jobid_col);
        let jobid = row.remove(jobid_col);
        row.insert(0, jobid);

        table_builder.add_record(row);
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
        .with(Modify::new(Rows::first()).with(MinWidth::new(Min)))
        .with(Width::wrap(width).keep_words())
        .with(Modify::new(Columns::single(0)).with(Format::new(|s| style(s).yellow().to_string())));

    println!("{table}");

    Ok(())
}
