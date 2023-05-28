mod cli;
mod tasks;

use anyhow::anyhow;
use cli::{CommandLineArgs, *};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Action::Add { task } => tasks::add_task(journal_file, Task::new(task)),
        Action::List => tasks::list_tasks(journal_file),
        Action::Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}

// cargo run -- -j test-journal.json add "buy milk"
// cargo run -- -j test-journal.json add "take the dog for a walk"
// cargo run -- -j test-journal.json add "water the plants"
// cargo run -- -j test-journal.json list
// cargo run -- -j test-journal.json done 2
