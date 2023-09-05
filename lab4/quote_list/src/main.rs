fn main() {
    println!("hello world");
}

fn quote(s: &str, c: char) -> String {
    format!("{}{}{}", c, s, c)
    }

fn quote_list(v: &[&str], c: char) -> Vec<String> {
    v.iter().map(|s| format!("{}{}{}", c, s, c)).collect()
}

fn quote_list_loop(v: &[&str], ch: char) -> Vec<String> {
    let mut result = Vec::with_capacity(v.len());
    for s in v{
        result.push(format!("{}{}{}",ch,s,ch))
    }
    result
}

fn quote_list_iter(arr: &[str],quote: char) -> Vec<String> {
    let mut result = Vec::new();
    let iter = arr.iter();

    while let Some(word) = iter.next() {
        let quoted = format!("{}{}{}",quote,word,quote);
        result.push(quoted);
    }
    result
}

fn quote_list_recursive(arr: &[&str], quote: char) -> Vec<String> {
    let mut result = Vec::new();

    if arr.is_empty() {
        return Vec::new();
    }
    let quoted = format!("{}{}{}",quote,arr[0],quote);
    if arr.len() >= 1 {
        result = quote_list_recursive(&arr[1..],'*');
    }
    result.insert(0,quoted);
    result
} 

#[test]
fn test_quotes() {
    assert_eq!(quote("abcd", '*'), "*abcd*");
    assert_eq!(quote_list(&[""; 0], '*'), &[""; 0]);
    assert_eq!(quote_list(&["abcd", "xyz"], '*'),["*abcd*", "*xyz*"]);
}

#[test]
fn test_quote_list() {
    assert_eq!(quote_list_loop(&["abcd", "xyz"], '*'),["*abcd*", "*xyz*"]);
    assert_eq!(quote_list_loop(&[""; 0], '*'), &[""; 0]);
}

#[test]
fn test_quote_list1() {
    assert_eq!(quote_list_iter(&["abcd", "xyz"], '*'),["*abcd*", "*xyz*"]);
    assert_eq!(quote_list_iter(&[""; 0], '*'), &[""; 0]);
}

#[test]
fn test_quote_list_recursive() {
    assert_eq!(quote_list_recursive(&["abcd", "xyz"], '*'),["*abcd*", "*xyz*"]);
    assert_eq!(quote_list_recursive(&[""; 0], '*'), &[""; 0]);
}