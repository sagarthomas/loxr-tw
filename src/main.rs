use std::{
    env, fs,
    io::{self, Write},
    process,
};

pub mod scanner;

// TODO: need to implement hadError global variable

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: loxr [script]");
        process::exit(64);
    } else if args.len() == 2 {
        if let Err(e) = run_file(&args[0]) {
            println!("Failed to execute: {e}");
            process::exit(1);
        }
    } else {
        if let Err(e) = run_prompt() {
            println!("Failed to execute: {e}");
            process::exit(1);
        }
    }
}

fn run_file(path: &String) -> Result<(), io::Error> {
    let contents = fs::read_to_string(path)?;
    run(contents);
    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        run(buffer)
    }
}

fn run(source: String) {
    //TODO: Add scanner
    println!("{source}")
}

fn error(line: i32, message: String) {
    report(line, "", message)
}

fn report(line: i32, location: &str, message: String) {
    println!("[line {line}] Error {location}: {message}")
}
