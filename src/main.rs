//raw Code No TUI or Gui for now
//Tasks Struct
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
#[derive(Debug, serde::Serialize, Deserialize, Clone)]
struct Task {
    text: String,
    completed: bool,
}

fn main() -> std::io::Result<()> {
    let mut tasks = load_tasks().unwrap();
    main_visuals(&mut tasks);
    Ok(())
}
fn main_visuals(mut tasks: &mut Vec<Task>) -> () {
    println!("Select Waht You Want To Do With Task");
    println!("1. add_task\n2. edit_task\n3. mark_done\n4. delete_task\n5. watch_tasks \n6.Quit");
    loop {
        let selected: i32 = shorter_input(true).parse().unwrap();
        match selected {
            1 => add_task_visuals(&mut tasks),
            2 => edit_task_visuals(&mut tasks),
            3 => mark_done_visuals(&mut tasks),
            4 => delete_task_visuals(&mut tasks),
            5 => watch_tasks(&tasks),
            6 => break,
            _ => println!("Wrong Number"),
        }
        let _ = saving(tasks);
    }
}
fn saving(tasks: &Vec<Task>) -> std::io::Result<()> {
    let file = File::create("results.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &tasks)?;
    writer.flush()
}
fn load_tasks() -> std::io::Result<Vec<Task>> {
    let path = "results.json";
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    // Відкриваємо файл для читання
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

fn add_task(tasks: &mut Vec<Task>, text: String) {
    let users_task = Task {
        text: text,
        completed: false,
    };
    tasks.push(users_task);
}

fn edit_task(tasks: &mut Vec<Task>, value: usize, input: String) {
    let new_name = Task {
        text: input,
        completed: false,
    };
    tasks[value] = new_name;
}

fn mark_done(tasks: &mut Vec<Task>, value: usize) {
    tasks[value].completed = true;
}
fn delete_task(tasks: &mut Vec<Task>, value: usize) {
    tasks.remove(value);
}
fn watch_tasks(tasks: &Vec<Task>) {
    println!("Heres Your Tasks {:?}", tasks);
}
fn shorter_input(must_be_number: bool) -> String {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("BimBimBamBam");
        let input = input.trim().to_string();
        if must_be_number == true {
            if input.parse::<i32>().is_ok() {
                return input;
            } else {
                println!("Error Wrong Input Settings");
            }
        } else {
            if !input.is_empty() {
                return input;
            } else {
                println!("Error Wrong Input Settings Use Only Letters");
            }
        }
    }
}
fn add_task_visuals(tasks: &mut Vec<Task>) -> () {
    println!("What The Name Of Task");
    let text = shorter_input(false).parse().unwrap();
    add_task(tasks, text);
    println!("Added New Task");
    println!("Oops Something Went Wrong");
}
fn edit_task_visuals(tasks: &mut Vec<Task>) -> () {
    if tasks.is_empty() {
        println!("You Doesnt Have Anything");
    } else {
        println!("Heres All You Have {:?}", tasks);
        println!("Which One You Want To Edit use only number ");
        let i: usize = shorter_input(true).parse().expect("BimBimBamBam");
        let i = i - 1;
        if i as usize >= tasks.len() {
            println!("You Selected Wrong Task");
            return;
        }
        println!("How Are you Want To Rename It");
        let new_input = shorter_input(false);
        edit_task(tasks, i, new_input);
    }
}
fn mark_done_visuals(tasks: &mut Vec<Task>) -> () {
    if tasks.is_empty() {
        println!("You Dont Have Tasks To mark As Done");
        println!("You Should Create New Task");
    } else {
        println!(
            "Heres All your tasks Which One You Want Mark As Done {:?}",
            tasks
        );
        println!("Which One You Want To Edit use only number ");
        let i: usize = shorter_input(true).parse().expect("BimBimBamBam");
        let i = i - 1;
        println!("Are You Sure You Want To mark as Done it (Y = yes N = no)");
        let x: String = shorter_input(false).parse().expect("BimBimBamBam");
        if x == "Y" {
            println!("Okay");
            //let mut new_completion = Task.completed = true;
            mark_done(tasks, i);
        } else {
            println!("Okay Xd");
        }
    }
}
fn delete_task_visuals(tasks: &mut Vec<Task>) -> () {
    if tasks.is_empty() {
        println!("Buddy You Dont Have Anything");
    } else {
        println!(
            "Heres All your tasks Which one you want to delete {:?}",
            tasks
        );
        println!("Which one you wana to deleate use only numbers");
        let i: usize = shorter_input(true).parse().expect("BimBimBamBam");
        let i = i - 1;
        println!("Are You Sure You Want To Delete It Couldnt Be Redone (Y = yes N = no");
        let x: String = shorter_input(false).parse().expect("BimBimBamBam");
        if x == "Y" {
            println!("Deleting");
            delete_task(tasks, i);
        } else {
            println!("Okay");
        }
    }
}
