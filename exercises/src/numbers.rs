use std::collections::HashMap;
use std::io;

pub fn run() {
    println!("find the median and mode of a string of numbers! type 'end' when you're done");
    let mut numbers = vec![];
    let mut freqs = HashMap::new();

    loop {
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        if num.trim() == "end" {
            break;
        }
        
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("that's not a valid number! try another one: ");
                continue;
            }
        };

        let count = freqs.entry(num).or_insert(0);
        *count += 1;

        numbers.push(num);
    }

    let mut mode = 0;
    let mut most_common = 0;

    numbers.sort();

    let median = numbers.get(numbers.len() / 2);
    match median {
        Some(median) => println!("The median is {median}"),
        None => println!("There is no median"),
    }

    for key in freqs.keys() {
        let opt = freqs.get(key);

        if let Some(val) = opt {
            if *val > most_common {
                mode = *key;
                most_common = *val;
            }
        };
    }

    println!("The mode is {mode}");
}