mod cli;
mod task;
use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;

fn find_default_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rust-todo-list.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Add { text } => task::add_task(journal_file, task::Task::new(text)),
        Done { position } => task::complete_task(journal_file, position),
        List => task::list_task(journal_file),
    }?;
    Ok(())
}
