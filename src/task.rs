use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Result, Seek, SeekFrom},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl Task {
    pub fn new(task: String) -> Task {
        Task {
            text: task,
            create_at: Utc::now(),
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let create_at = self.create_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, create_at)
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = get_task_in_file(&file)?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    println!("Task added [{} task(s) to complete]", tasks.len());
    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = get_task_in_file(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "Invalid input taskID",
        ));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    println!(
        "Task complete and deleted [{} task(s) to complete]",
        tasks.len()
    );
    Ok(())
}

pub fn list_task(journal_path: PathBuf) -> Result<()> {
    let file: File = OpenOptions::new().read(true).open(journal_path)?;

    let tasks: Vec<Task> = get_task_in_file(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty");
    } else {
        let mut count = 1;
        for task in tasks {
            println!("Task #{}: {}", count, task);
            count += 1;
        }
    }

    Ok(())
}

fn get_task_in_file(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;

    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}
