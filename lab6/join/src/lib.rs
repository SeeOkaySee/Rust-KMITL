fn join_strings(words: &[&str],sep: &str) -> String {
    let mut iter = words.iter();
    let mut result = String::new();
    let mut count = 0;
    let size = words.len();

    while let Some(word) = iter.next(){
        if size < 2 {
            result.push_str(&format!("{}",word));
        }
        else if count == size - 1 {
            result.push_str(&format!("{}",word));
        } else {
            result.push_str(&format!("{}{}",word,sep));
            count += 1
        }
    }
    result
}

fn join_numbers(numbers: &[i32],sep: &str) -> String {
    let mut iter = numbers.iter();
    let mut result = String::new();
    let mut count = 0;
    let size = numbers.len();

    while let Some(number) = iter.next() {
        if size < 2 {
            result.push_str(&format!("{}",number));
        }
        else if count == size - 1 {
            result.push_str(&format!("{}",number));
        }
        else {
            result.push_str(&format!("{}{}",number,sep));
            count += 1
        }
    }
    result
}

#[test]
fn test_join_strings() {
    assert_eq!(join_strings(&[], ","), "");
    assert_eq!(join_strings(&["C"], ","), "C");
    let patterns = ["C", "Rust", "C++", "Python"];
    assert_eq!(join_strings(&patterns, ", "), "C, Rust, C++, Python");
    assert_eq!(join_strings(&patterns, ";;"), "C;;Rust;;C++;;Python");
}

#[test]
fn test_join_numbers() {
assert_eq!(join_numbers(&[], ","), "");
assert_eq!(join_numbers(&[25], ","), "25");
let patterns = [5, 10, -1, 2];
assert_eq!(join_numbers(&patterns, ", "), "5, 10, -1, 2");
assert_eq!(join_numbers(&patterns, ";;"), "5;;10;;-1;;2");
}