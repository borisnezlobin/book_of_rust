#[cfg(test)]
pub mod tests {
    use crate::pig_latin::{translate_word, de_translate, translate_sentence};

    #[test]
    fn test_hello_world() {
        assert_eq!(translate_word("hello"), "ellohay");
    }

    #[test]
    fn test_de_translate(){
        assert_eq!(de_translate("ellohay"), "hello");
    }

    #[test]
    fn test_sentence_with_no_words(){
        assert_eq!(translate_sentence("hello"), "ellohay");
    }

    #[test]
    fn test_sentence(){
        assert_eq!(translate_sentence("hello world"), "ellohay orldway");
    }
}
