use std::env;
use std::fs;

fn print_help() {
    println!("minigrep help");
    println!("USAGE:");
    println!("\tminigrep {{filename}} {{searchstring}}");
}

fn search<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let mut vec = vec![];
    for line in content.lines() {
        if line.contains(query){
            vec.push(line);
        }
    }

    vec
}

// the chapter goes on to reformat and stuff... nah I ain't doing all that
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || (args.len() == 2 && args[1] == "--help") {
        print_help();
        return;
    }

    if args.len() != 3 {
        let error = if args.len() < 3 { "few" } else { "many" };
        println!("ERROR: too {error} arguments");
        print_help();
        return;
    }

    let fname = &args[1];
    let query = &args[2];

    // dbg!("reading file \"{fname}\" and searching for \"{query}\"");

    let contents = fs::read_to_string(fname).expect("failed to read file");

    let lines = search(&contents, query);

    for line in lines {
        println!("{}", line);
    }
}