fn digit_count1(s: &str) -> usize{
    let digit = s.chars();
    let mut count = 0;
    for i in digit {
        if i.is_digit(10) {
            count += 1
        }
    }
    return count;
}

fn digit_count1_r(s: &str) -> usize {
    let mut count = 0;
    let mut digit = s.chars();
    if s.is_empty() {
        return 0
    }

    let first_char = digit.next().unwrap();
    let all_char = &s[1..];

    if first_char.is_digit(10) {
        return 1+digit_count1_r(all_char);
    } 
    else {
        return digit_count1_r(all_char)
    }
}

fn count_digits_v2(s: &str) -> Vec<(String, usize)> {
    let mut result = Vec::new();
    for letters in s.split_whitespace() {
        let mut count = 0;
        let digit = letters.chars();
        for i in digit {
            if i.is_digit(10) {
                count += 1;
            }
        }
        result.push((letters.to_string(), count));
    }
    return result;
}

#[test]
fn test_digits_count1() {
    assert_eq!(digit_count1(""), 0);
    assert_eq!(digit_count1("abcd"), 0);
    assert_eq!(digit_count1("ab12xy5 7x83y5z"), 7);
    assert_eq!(digit_count1("qwer127820 asdf890 8q 3"), 11);
    assert_eq!(digit_count1("ajof0209283098234aodfjiaoijw002jewrt 9999999"), 23);
}

#[test]
fn test_digits_count1_recursion() {
    assert_eq!(digit_count1_r(""), 0);
    assert_eq!(digit_count1_r("abcd"), 0);
    assert_eq!(digit_count1_r("ab12xy5 7x83y5z"), 7);
    assert_eq!(digit_count1_r("qwer127820 asdf890 8q 3"), 11);
    assert_eq!(digit_count1_r("ajof0209283098234aodfjiaoijw002jewrt 9999999"), 23);
}

#[test]
fn test_digits_count2() {
assert_eq!(count_digits_v2(""), []);
assert_eq!(
count_digits_v2("ab12xy5 7x83y5z"),
[
("ab12xy5".to_string(), 3), // '1', '2', '5'
("7x83y5z".to_string(), 4) // '7', '8', '3', '5'
]
);
assert_eq!(count_digits_v2("pqosd102839 9723asd 23uu"), [("pqosd102839".to_string(), 6), ("9723asd".to_string(), 4), ("23uu".to_string(), 2)]);
}