//raw Code No TUI or Gui for now
//Tasks Struct
use std::io;
#[derive(Debug)]
struct Task {
    text: String,
    completed: bool,
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    println!("Hello World XDXDDXDXD");
    loop {
        println!(
            "1. For New Task \n2. For Edit Task or add a Path \n3. For Marking As Done \n4. For Deleting Task \n5. Quit"
        );
        let selected = shorter_input().parse().expect("BimBimBamBam");
        match selected {
            1 => add_task(&mut tasks),
            2 => edit_task(&mut tasks),
            3 => mark_done(&mut tasks,),
            5 => break,
            _ => println!("Wrong Number"),
        }
    }
}
fn add_task(tasks: &mut Vec<Task>) {
    let mut new_task = String::new();
    io::stdin().read_line(&mut new_task).expect("Error");
    let main_task = new_task.trim().to_string();
    let users_task = Task {
        text: main_task,
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
        let i: u8 = shorter_input().parse().expect("BimBimBamBam");
        println!("How Are you Want To Rename It");
        let new_input = shorter_input();
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
    }
    else {
        println!("Heres All your tasks Which One You Want Mark As Done {:?}", tasks);
        println!("Which One You Want To Edit use only number ");
        let i: u8 = shorter_input().parse().expect("BimBimBamBam");
        println!("Are You Sure You Want To mark as Done it (Y = yes N = no)");
        let x: String = shorter_input().parse().expect("BimBimBamBam");
        if x == "Y" {
            println!("Okay");
            //let mut new_completion = Task.completed = true;
            tasks[i as usize].completed = true;
        }
        else {
            println!("Okay Xd");
        }
    }
}

fn shorter_input() -> String {
    //here i was lazy to write it everytime
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("BimBimBamBam");
    input.trim().to_string()
}
