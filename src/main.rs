use std::io::{self, BufRead};

struct Task {
    pub name: String,
    pub is_completed: bool,
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
    loop {
        list_tasks(&tasks);

        let option = readline();
        _ = match option.as_str() {
            "1" => {
                println!("Enter new task name: ");
                let name = readline();
                add_task(&mut tasks, name);
            },
            "2" => {
                println!("Enter task to complete: ");
                let level: i32 = readline().parse::<i32>().unwrap();
                complete_task(&mut tasks, level);
            },
            _ => break,
        };
    }
    Ok(())
}

fn add_task(tasks: &mut Vec<Task>, name: String) -> io::Result<()> {
    tasks.push(Task {
        name: name,
        is_completed: false,
    });
    Ok(())
}

fn list_tasks(tasks: &Vec<Task>) {
    for _ in 0..50 {
        println!("\n");
    }
    println!("Tasks List: ");
    for task in tasks {
        println!("Name: {}", task.name);
        println!("Is Completed: {}", task.is_completed);
    }
    println!("Choose the following options:
1. Add tasks
2. Complete tasks
3. Exit");
}

fn complete_task(tasks: &mut Vec<Task>, level: i32) -> io::Result<()> {
    tasks[level as usize].is_completed = true;
    Ok(())
}

