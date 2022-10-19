mod cli;
mod task;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file.expect("Failed to find journal file");

    match action {
        Add { text } => task::add_task(journal_file, task::Task::new(text)),
        Done { position } => task::complete_task(journal_file, position),
        List => task::list_task(journal_file),
    }
    .expect("Failed to perform action")
}
