use colored::Colorize;
use std::env;
use std::fs;
use std::process;

fn print_help() {
    println!("minigrep help");
    println!("USAGE:");
    println!(
        "\tminigrep {} {}",
        "filename".purple(),
        "search_string".blue()
    );
}

fn search<'a>(content: &'a str, query: &str) -> Vec<(&'a str, i32)> {
    let mut vec: Vec<(&str, i32)> = vec![];
    for (i, line) in content.lines().enumerate() {
        if line.contains(query) {
            vec.push((line, (i as i32) + 1));
        }
    }

    vec
}

// the chapter goes on to reformat and stuff... nah I ain't doing all that
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || (args.len() == 2 && args[1] == "--help") {
        print_help();
        process::exit(0);
    }

    if args.len() != 3 {
        let error = if args.len() < 3 { "few" } else { "many" };
        eprintln!("{}: too {} arguments", "ERROR".red(), error);
        print_help();
        process::exit(1);
    }

    let fname = &args[1];
    let query = &args[2];

    // dbg!("reading file \"{fname}\" and searching for \"{query}\"");

    let contents = fs::read_to_string(fname).expect("failed to read file");

    let lines = search(&contents, query);

    for line in lines {
        println!("{} {}", format!("{}:", line.1).green().italic(), line.0);
    }
}
