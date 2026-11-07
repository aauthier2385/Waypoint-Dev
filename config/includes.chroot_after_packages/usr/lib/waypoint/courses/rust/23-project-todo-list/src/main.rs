use std::env;
use std::path::Path;
use std::process;

use todo::TodoList;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new("todo.txt");

    let mut list = match TodoList::load(path) {
        Ok(list) => list,
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    };

    let command = args.get(1).map(String::as_str).unwrap_or("list");
    let result = match command {
        "add" => list.add(&args[2..].join(" ")),
        "done" => number(&args).and_then(|n| list.complete(n)),
        "remove" => number(&args).and_then(|n| list.remove(n)).map(|task| {
            println!("Removed: {}", task.title);
        }),
        "list" => Ok(()),
        _ => Err(format!("I don't know \"{command}\". Try: add, done, remove, or list")),
    };

    if let Err(message) = result {
        eprintln!("{message}");
        process::exit(1);
    }
    if let Err(message) = list.save(path) {
        eprintln!("{message}");
        process::exit(1);
    }

    for (index, task) in list.tasks.iter().enumerate() {
        let mark = if task.done { "x" } else { " " };
        println!("{}. [{mark}] {}", index + 1, task.title);
    }
    println!("{}", list.summary());
}

/// Reads the task number typed after "done" or "remove".
fn number(args: &[String]) -> Result<usize, String> {
    let text = args.get(2).ok_or("Which task? For example: done 1")?;
    text.parse().map_err(|_| format!("\"{text}\" isn't a task number"))
}
