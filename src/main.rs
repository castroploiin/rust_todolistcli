use csv::{Reader, StringRecord, WriterBuilder};
use std::fs;
use std::io::{self, LineWriter, Write};
use std::path::Path;

fn main() -> Result<(), io::Error> {
    loop {
        let input = get_input("Enter command: ")?;
        parse_input(input)?;
    }
}

#[derive(serde::Serialize)]
struct Task<'a> {
    title: &'a str,
}

fn parse_input(arg: String) -> Result<(), io::Error> {
    match arg.trim() {
        "add" => add_task()?,
        "complete" => complete_task()?,
        "clear" => clear_tasks()?,
        "restore" => restore_tasks()?,
        "list" => list_tasks()?,
        "quit" => std::process::exit(69),
        _ => return Ok(()),
    }
    Ok(())
}

fn get_input(with_guide: &str) -> Result<String, io::Error> {
    println!("{}", with_guide);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn add_task() -> Result<(), io::Error> {
    let task_result = get_input("Enter the name of the task:")?;
    let task = task_result.trim().to_owned() + "\n";

    let file_content = fs::read_to_string(Path::new("src/todos.csv"))?;
    let file = open_file()?;
    let mut writer = LineWriter::new(file);

    writer.write_all((file_content + &task).as_bytes())?;
    Ok(())
}

fn complete_task() -> Result<(), io::Error> {
    let task_name = get_input("Enter the task to be marked complete:")?;

    bufferize()?;
    copy_without(Some(&task_name))?;

    println!("Task '{}' marked as complete\n", &task_name.trim());
    Ok(())
}

fn clear_tasks() -> Result<(), io::Error> {
    bufferize()?;
    fs::File::create(Path::new("src/todos.csv"))?;

    Ok(())
}

fn restore_tasks() -> Result<(), io::Error> {
    copy_without(None)
}

fn list_tasks() -> Result<(), io::Error> {
    let task_list = fs::read_to_string(Path::new("src/todos.csv"))?;
    let mut task_list_vector: Vec<&str> = Vec::new();

    for task in task_list.split('\n') {
        if task != "" && task != "title" {
            task_list_vector.push(task);
        }
    }

    println!("Tasks:");
    for task in task_list_vector {
        println!("{}", task)
    }

    Ok(())
}

fn bufferize() -> Result<(), io::Error> {
    let todo_file = Path::new("src/todos.csv");
    let buffer_file = Path::new("src/buffer.csv");

    fs::copy(todo_file, buffer_file)?;
    Ok(())
}

fn copy_without(task_name: Option<&String>) -> Result<(), io::Error> {
    let mut buffer_file = Reader::from_path(Path::new("src/buffer.csv"))?;
    let mut todo_file = WriterBuilder::new().from_path(Path::new("src/todos.csv"))?;

    for line in buffer_file.records() {
        let unwrap_line = line?;
        match task_name {
            Some(task) => {
                let true_instance = StringRecord::from(vec![format!("{}", task.trim().to_owned())]);
                let false_instance =
                StringRecord::from(vec![format!("{}", task.trim().to_owned())]);

                if &unwrap_line != &true_instance && &unwrap_line != &false_instance {
                    let task: Task = Task {
                        title: &unwrap_line[0],
                    };
                    todo_file.serialize(task)?;
                }
            },
            None =>  {
                let mut buffer_file = Reader::from_path(Path::new("src/buffer.csv"))?;
                let mut todo_file = WriterBuilder::new().from_path(Path::new("src/todos.csv"))?;

                for line in buffer_file.records() {
                    let unwrap_line = line?;
                    let task: Task = Task {
                        title: &unwrap_line[0],
                    };
                    todo_file.serialize(task)?;
                }
            }
        }
    }

    Ok(())
}

fn open_file() -> Result<fs::File, std::io::Error> {
    let file_path = Path::new("src/todos.csv");
    fs::File::create(file_path)
}
