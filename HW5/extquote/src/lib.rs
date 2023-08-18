fn extract_quoted_words(text: &str) -> Vec<String> {
    let mut quoted = Vec::new();

    for word in text.split_whitespace() {
        if word.starts_with("*") && word.ends_with("*") {
            let mut quotedword = word.trim_start_matches("*").to_string();
            quotedword = quotedword.trim_end_matches("*").to_string();
            quoted.push(word.to_string());
        }
    }
    quoted
}

fn extract_quoted_words_r(text: &str) -> Vec<String> {
    fn extract_rec(iter: &mut std::str::SplitWhitespace, quoted: &mut Vec<String>) {
        if let Some(word) = iter.next() {
            if word.starts_with("*") && word.ends_with("*") {
                let mut quoted_word = word.trim_start_matches("*").to_string();
                quoted_word = quoted_word.trim_end_matches("*").to_string();
                quoted.push(quoted_word);
            }
            extract_rec(iter, quoted);
        }
    }

    let mut quoted = Vec::new();
    let mut iter = text.split_whitespace();
    extract_rec(&mut iter, &mut quoted);
    quoted
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        vec!["**", "*C++*", "*Python*"]
    );
}

#[test]
fn test_extract_quoted_words_r() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new());
    assert_eq!(
        extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        vec!["**", "*C++*", "*Python*"]
    );
}