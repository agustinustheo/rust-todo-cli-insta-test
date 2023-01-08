use std::env;
use std::io::{self, BufRead};
use std::path::Path;

struct Task {
    name: String,
    is_completed: bool,
}

fn readline() -> String {
    let mut strr: String = "".to_string();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        strr = line.unwrap().to_string();
        break;
    }
    strr
}

fn main() -> io::Result<()> {
    let mut tasks: Vec<Task> = vec![];
    while true {
        println!("{}", readline());
        break;
    }
    Ok(())
}

fn add_task(tasks: &Vec<Task>) -> io::Result<()> {
    // TODO: Add logic
    Ok(())
}

fn list_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    // TODO: List logic
    Ok(())
}

fn complete_task(tasks: &Vec<Task>) -> io::Result<()> {
    // TODO: Complete logic
    Ok(())
}

