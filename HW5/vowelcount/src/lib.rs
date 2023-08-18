fn count_vowels(n: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut count = 0;
    let word = n.chars();
    
    for i in word {
        if vowels.contains(&i) {
            count += 1;
        }
    }
    count
}

fn count_vowels_r(s: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    if s.is_empty() {
        return 0;
    }

    let first_char = s.chars().next().unwrap();
    let remaining_chars = &s[first_char.len_utf8()..];

    let count = if vowels.contains(&first_char) {
        1
    } else {
        0
    };

    count + count_vowels_r(remaining_chars)
}

fn count_vowels_v2(s: &str) -> Vec<(String, usize)> {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut result = Vec::new();

    for letters in s.split_whitespace() {
        let mut count = 0;
        let word = letters.chars();

        for i in word {
            if vowels.contains(&i) {
                count += 1;
            }
        }
        result.push((letters.to_string(), count));
    }
    return result;
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
}

#[test]
fn test_vowels_count_r() {
    assert_eq!(count_vowels_r(""), 0);
    assert_eq!(count_vowels_r("abEcd"), 2);
    assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
}

#[test]
fn test_vowels_count2() {
assert_eq!(count_vowels_v2(""), []);
assert_eq!(
count_vowels_v2("ab12Exey5 7x8U3y5z"),
[
("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
("7x8U3y5z".to_string(), 1) // 'U'
]
);
}