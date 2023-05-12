use std::fs;

use std::io;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;

use csv::Reader;
use csv::StringRecord;

use csv::WriterBuilder;


fn main() -> Result<(), io::Error> {
    // WORKS!
    // copy_without("hello");
    // copy_without("again");
    // copy_without("andagain");

    let input = get_input("Enter command: ")?;
    parse_input(input)?;
    list_tasks()?;

    Ok(())
}

#[derive(serde::Serialize)]
struct Task<'a> { 
    title: &'a str, 
}

fn parse_input(arg: String) -> Result<(), io::Error> {
    // dbg!(&arg);
    match arg.trim() {
        "add" => add_task()?,
        "complete" => complete_task()?,
        "list" => list_tasks()?,
        _ => return Ok(())
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
    // println!();
    let task_result = get_input("Enter the name of the task:")?;
    let task = task_result.trim().to_owned() + "\n";

    let file_content = fs::read_to_string(Path::new("src/todos.csv"))?;
    let file = open_file()?;
    // let file_content = fs::read_to_string(Path::new("src/todos.csv"))?;
    let mut writer = LineWriter::new(file);
    
    // dbg!(file_content.clone());
    // dbg!(file_content.clone() + &task);
    // file_content + &task;
    writer.write_all((file_content + &task).as_bytes())?;

    Ok(())
}

fn complete_task() -> Result<(), io::Error> {
    let task_name = get_input("Enter the task to be marked complete:")?;

    // Bufferize file and copy it back to todos.csv until the offending line is read, which is skipped, following the continuation of the copying
    bufferize()?;
    copy_without(&task_name)?;

    println!("Task '{}' marked as complete", &task_name.trim());
    Ok(())
}

fn list_tasks() -> Result<(), io::Error> {
    let task_list = fs::read_to_string(Path::new("src/todos.csv"))?;
    let mut task_list_vector: Vec<&str> = Vec::new();

    for task in task_list.split('\n') {
        task_list_vector.push(task);
    }

    // dbg!(task_list_vector);
    Ok(())
}

fn bufferize() -> Result<(), io::Error> {
    let todo_file = Path::new("src/todos.csv");
    let buffer_file = Path::new("src/buffer.csv");

    fs::copy(todo_file, buffer_file)?;
    Ok(())
}

fn copy_without(task_name: &String) -> Result<(), io::Error> {
    bufferize()?;
    let mut buffer_file = Reader::from_path(Path::new("src/buffer.csv"))?;
    let mut todo_file = WriterBuilder::new().from_path(Path::new("src/todos.csv"))?;

    for line in buffer_file.records() {
        let unwrap_line = line?;
        let true_instance = StringRecord::from(vec![format!("{}", task_name.trim().to_owned())]);
        let false_instance = StringRecord::from(vec![format!("{}", task_name.trim().to_owned())]);
        dbg!(&unwrap_line);

        if &unwrap_line != &true_instance && &unwrap_line != &false_instance {
            let task: Task = Task { title: &unwrap_line[0] };
            todo_file.serialize(task)?;
        }
    }

    Ok(())
}

fn open_file() -> Result<fs::File, std::io::Error> {
    let file_path = Path::new("src/todos.csv");
    match file_path.exists() {
        true => { 
            let file = fs::File::create(file_path);
            return file
        },
        false => {
            // fs::File::create(file_path)?;
            let file = fs::File::create(file_path);

            return file
        }
    }
}
