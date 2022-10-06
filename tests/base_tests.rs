use wordsplit::{until_char, WordSplit};

#[test]
fn it_works() {
    let haystack = "a b c d";
    let delimiter = " ";
    let letters = WordSplit::new(haystack, delimiter);
    assert!(letters.eq(vec!["a", "b", "c", "d"].into_iter()));
}

#[test]
fn haystack_tail() {
    let haystack = "a b c d ";
    let delimiter = " ";
    let letters = WordSplit::new(haystack, delimiter);
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}

#[test]
fn until_char_test() {
    let sentence = "hello rust";
    let char = 'o';
    assert_eq!(until_char(sentence, char), "hell");
}

#[test]
fn str_split_by_underscore() {
    let haystack = "hallo_mein_name_ist_roy";
    let delimiter = "_";
    let letters: Vec<_> = WordSplit::new(haystack, delimiter).collect();
    assert_eq!(letters, vec!["hallo", "mein", "name", "ist", "roy"]);
}

#[test]
fn split_sentence_by_word() {
    let haystack = "I am good. I am better. I am great.";
    let delimiter = "am";
    let letters: Vec<_> = WordSplit::new(haystack, delimiter).collect();
    assert_eq!(letters, vec!["I ", " good. I ", " better. I ", " great."]);
}
