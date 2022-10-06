use wordsplit::{until_char, WordSplit};
#[test]
fn it_should_not_work() {
    let haystack = "a b c d";
    let delimiter = " ";
    let letters = WordSplit::new(haystack, delimiter);
    assert!(letters.ne(vec!["a", "b", "c"].into_iter()));
}

#[test]
fn neg_until_char_test() {
    let sentence = "hello rust";
    let char = 'o';
    assert_ne!(until_char(sentence, char), "he")
}

#[test]
fn neg_haystack_tail() {
    let haystack = "a b c d ";
    let delimiter = " ";
    let letters = WordSplit::new(haystack, delimiter);
    assert!(letters.ne(vec!["a", "b", "c", "d"].into_iter()));
}

#[test]
fn neg_str_split_by_underscore() {
    let haystack = "hallo_mein_name_ist_roy";
    let delimiter = "_";
    let letters: Vec<_> = WordSplit::new(haystack, delimiter).collect();
    assert_ne!(letters, vec!["hallo_mein", "name", "ist", "roy"]);
}

#[test]
fn neg_split_sentence_by_word() {
    let haystack = "I am good. I am better. I am great.";
    let delimiter = "am";
    let letters: Vec<_> = WordSplit::new(haystack, delimiter).collect();
    assert_ne!(letters, vec!["I", "good. I", "better. I", "great."]);
}
