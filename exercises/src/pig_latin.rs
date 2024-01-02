
pub fn translate_word(word: &str) -> String {
    let mut new_word = String::new();
    let chars: Vec<char> = word.chars().collect();

    for &c in chars.iter().skip(1) {
        new_word.push(c);
    }

    new_word.push(chars[0]);
    new_word.push_str("ay");

    new_word
}

pub fn de_translate(word: &str) -> String {
    let mut new_word = String::new();
    let chars: Vec<char> = word.chars().collect();
    new_word.push(chars[chars.len() - 3]);

    for &c in chars.iter().rev().skip(3).rev() {
        new_word.push(c);
    }

    new_word
}

pub fn translate_sentence(sentence: &str) -> String {
    let mut new_sentence = String::new();
    let words: Vec<&str> = sentence.split(" ").collect();

    for i in 0..words.len() {
        new_sentence.push_str(&translate_word(words[i]));
        if i != words.len() - 1 { new_sentence.push(' ') };
    }

    return new_sentence;
}