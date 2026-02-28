//raw Code No TUI or Gui for now
//Tasks Struct
use std::io;
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
            2 => println!("Not Done Currently"),
            5 => break,
            _ => println!("Wrong Number"),
        }
    }
}
fn add_task(tasks: &mut Vec<Task>) {
    let mut New_Task = String::new();
    io::stdin().read_line(&mut New_Task).expect("Error");
    let mut Maded_Task = New_Task.trim().to_string();
    let mut Users_Task = Task {
        text: Maded_Task,
        completed: false,
    };
    tasks.push(Users_Task);
    println!("Added New Task");
}

fn shorter_input() -> String {
    //here i was lazy to write it everytime
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("BimBimBamBam");
    input.trim().to_string()
}
