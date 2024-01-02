use crate::pig_latin::translate_word;
use crate::pig_latin::de_translate;

pub mod numbers;
pub mod pig_latin;

#[cfg(test)]
pub mod tests;

fn main() {
    println!("Hello, world!");
    println!("{}", translate_word("world"));
    println!("{}", de_translate("ellohay"));
    numbers::run();
}
