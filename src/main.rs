use std::io::{self, BufRead};

// Making to structs like this isn't
// best practice for production
// it's here only to simplify things
#[cfg(test)]
#[derive(serde::Serialize, Clone)]
struct Task {
    pub name: String,
    pub is_completed: bool,
}

#[cfg(not(test))]
#[derive(Clone)]
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
        match option.as_str() {
            "1" => {
                println!("Enter new task name: ");
                let name = readline();
                _ = add_task(&mut tasks, name);
            },
            "2" => {
                println!("Enter task to complete: ");
                let level: i32 = readline().parse::<i32>().unwrap();
                _ = complete_task(&mut tasks, level);
            },
            _ => break,
        };
    }
    Ok(())
}

fn add_task(tasks: &mut Vec<Task>, name: String) -> io::Result<Task> {
    let task = Task {
        name: name,
        is_completed: false,
    };
    tasks.push(task.clone());
    Ok(task)
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

fn complete_task(tasks: &mut Vec<Task>, level: i32) -> io::Result<Task> {
    tasks[level as usize].is_completed = true;
    Ok(tasks[level as usize].clone())
}

#[cfg(test)]
extern crate insta;

#[cfg(test)]
mod tests {
    use super::*;
    use insta::{assert_json_snapshot, assert_compact_json_snapshot, assert_csv_snapshot};

    #[test]
    fn test_json_task_struct() {
        let task: Task = Task {
            name: "name".to_string(),
            is_completed: false,
        };
        assert_json_snapshot!(task, @r###"{
  "name": "name",
  "is_completed": false
}"###);
        assert_compact_json_snapshot!(task, @r###"{"name": "name", "is_completed": false}"###);
    }

    #[test]
    fn test_json_task_empty_struct_vec() {
        let tasks: Vec<Task> = vec![];
        assert_json_snapshot!(tasks, @r###"[]"###);
    }

    #[test]
    fn test_json_task_struct_vec() {
        let tasks: Vec<Task> = vec![Task {
            name: "name".to_string(),
            is_completed: false,
        }];
        assert_json_snapshot!(tasks, @r###"[
  {
    "name": "name",
    "is_completed": false
  }
]"###);
        assert_compact_json_snapshot!(tasks, @r###"[{"name": "name", "is_completed": false}]"###);
    }

    #[test]
    fn test_json_add_task_struct_vec() {
        let mut tasks: Vec<Task> = vec![];

        let task: Task = add_task(&mut tasks, "name".to_string()).unwrap();
        assert_json_snapshot!(task, @r###"{
  "name": "name",
  "is_completed": false
}"###);
        assert_compact_json_snapshot!(task, @r###"{"name": "name", "is_completed": false}"###);

        assert_json_snapshot!(tasks, @r###"[
  {
    "name": "name",
    "is_completed": false
  }
]"###);
        assert_compact_json_snapshot!(tasks, @r###"[{"name": "name", "is_completed": false}]"###);
    }

    #[test]
    fn test_json_complete_task_struct_vec() {
        let mut tasks: Vec<Task> = vec![];

        let task: Task = add_task(&mut tasks, "name".to_string()).unwrap();
        assert_json_snapshot!(task, @r###"{
  "name": "name",
  "is_completed": false
}"###);
        assert_compact_json_snapshot!(task, @r###"{"name": "name", "is_completed": false}"###);

        assert_json_snapshot!(tasks, @r###"[
  {
    "name": "name",
    "is_completed": false
  }
]"###);
        assert_compact_json_snapshot!(tasks, @r###"[{"name": "name", "is_completed": false}]"###);

        let task: Task = complete_task(&mut tasks, 0).unwrap();
        assert_json_snapshot!(task, @r###"{
  "name": "name",
  "is_completed": true
}"###);
        assert_compact_json_snapshot!(task, @r###"{"name": "name", "is_completed": true}"###);

        assert_json_snapshot!(tasks, @r###"[
  {
    "name": "name",
    "is_completed": true
  }
]"###);
        assert_compact_json_snapshot!(tasks, @r###"[{"name": "name", "is_completed": true}]"###);
    }

    #[test]
    fn test_csv_task_struct() {
        let task: Task = Task {
            name: "name".to_string(),
            is_completed: false,
        };
        assert_csv_snapshot!(task, @r###"name,is_completed
name,false"###);
    }

    #[test]
    fn test_csv_task_empty_struct_vec() {
        let tasks: Vec<Task> = vec![];
        assert_csv_snapshot!(tasks, @r###""###);
    }

    #[test]
    fn test_csv_task_struct_vec() {
        let tasks: Vec<Task> = vec![Task {
            name: "name".to_string(),
            is_completed: false,
        },Task {
            name: "name2".to_string(),
            is_completed: false,
        }];
        assert_csv_snapshot!(tasks, @r###"name,is_completed
name,false
name2,false"###);
    }
}
