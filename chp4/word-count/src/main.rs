use std::io;

fn main() {
    println!("Enter a text: ");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read line");
    let (lines, words, chars) = word_count(&text);
    println!("lines: {}\nwords: {}\nchars: {}\n", lines, words, chars);
}

// counts lines, words, and characters in a string
fn word_count(s: &str) -> (i32, i32, i32) {
    if s.len() == 0 {
        return (0, 0, 0);
    }
    let mut lines = 1;
    let mut words = 1;
    let mut chars = 0;
    let bytes = s.as_bytes();
    for (i , &item) in bytes.iter().enumerate() {
        // NOTE end-of-line is LF or ASCII line-feed character on mac or 10 in base10

        // going through each char
        // if (char is newline) +1 line
        if item == b'\n' {
            lines += 1;
        }
        // if (char is whitespace, tab, newline, or line-feed) +1 word
        if item == b' ' || item == b'\t' || item == b'\n' {
            words += 1;
        }
        // if (char is not whitespace, tab, newline, or line-feed) +1 char
        if item != 10 {
            chars += 1;
        }

        println!("{} {} {}", i, item, char::from(item));
    }
    // add +1 to line
    // return (lines, words, chars)
    (lines, words, chars)
}

#[test]
fn test_word_count() {
    assert_eq!(word_count(""), (0,0,0));
    assert_eq!(word_count("abc"), (1, 1, 3));
    assert_eq!(word_count("abc\ndef"), (2, 2, 6));
    assert_eq!(word_count("abc def"), (1, 2, 7));
    assert_eq!(word_count("abc\ndef\n"), (3, 2, 7));
}
