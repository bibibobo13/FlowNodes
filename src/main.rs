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
    let mut tasks: Vec<Task> = if Path::new("results.json").exists() {
        let file = File::open("results.json")?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)?
    } else {
        Vec::new()
    };
    loop {
        println!(
            "1. For New Task \n2. For Edit Task or add a Path \n3. For Marking As Done \n4. For Deleting Task \n5. Watch Tasks \n6. Quit"
        );
        let selected: i32 = shorter_input(true).parse().unwrap();
        match selected {
            1 => {
                add_task(&mut tasks);
                let file = File::create("results.json")?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &tasks)?;
                writer.flush()?;
            }
            2 => {
                edit_task(&mut tasks);
                let file = File::create("results.json")?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &tasks)?;
                writer.flush()?;
            }
            3 => {
                mark_done(&mut tasks);
                let file = File::create("results.json")?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &tasks)?;
                writer.flush()?;
            }
            4 => {
                delete_task(&mut tasks);
                let file = File::create("results.json")?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &tasks)?;
                writer.flush()?;
            }
            5 => {
                watch_tasks(&tasks);
                let file = File::create("results.json")?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &tasks)?;
                writer.flush()?;
            }
            6 => {
                let file = File::create("results.json")?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer(&mut writer, &tasks)?;
                break writer.flush();
            }
            _ => println!("Wrong Number"),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let new_task = shorter_input(false).parse().unwrap();
    let users_task = Task {
        text: new_task,
        completed: false,
    };
    tasks.push(users_task);
    println!("Added New Task");
}

fn edit_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("You Doesnt Have Anything");
    } else {
        println!("Heres All You Have {:?}", tasks);
        println!("Which One You Want To Edit use only number ");
        let i: u8 = shorter_input(true).parse().expect("BimBimBamBam");
        let i = i - 1;
        if i as usize >= tasks.len() {
            println!("You Selected Wrong Task");
            return;
        }
        println!("How Are you Want To Rename It");
        let new_input = shorter_input(false);
        let new_name = Task {
            text: new_input,
            completed: false,
        };
        tasks[i as usize] = new_name;
    }
}

fn mark_done(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("You Dont Have Tasks To mark As Done");
        println!("You Should Create New Task");
    } else {
        println!(
            "Heres All your tasks Which One You Want Mark As Done {:?}",
            tasks
        );
        println!("Which One You Want To Edit use only number ");
        let i: u8 = shorter_input(true).parse().expect("BimBimBamBam");
        let i = i - 1;
        println!("Are You Sure You Want To mark as Done it (Y = yes N = no)");
        let x: String = shorter_input(false).parse().expect("BimBimBamBam");
        if x == "Y" {
            println!("Okay");
            //let mut new_completion = Task.completed = true;
            tasks[i as usize].completed = true;
        } else {
            println!("Okay Xd");
        }
    }
}
fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("Buddy You Dont Have Anything");
    } else {
        println!(
            "Heres All your tasks Which one you want to delete {:?}",
            tasks
        );
        println!("Which one you wana to deleate use only numbers");
        let i: u8 = shorter_input(true).parse().expect("BimBimBamBam");
        let i = i - 1;
        println!("Are You Sure You Want To Delete It Couldnt Be Redone (Y = yes N = no");
        let x: String = shorter_input(false).parse().expect("BimBimBamBam");
        if x == "Y" {
            println!("Deleting");
            tasks.remove(i as usize);
        } else {
            println!("Okay");
        }
    }
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
