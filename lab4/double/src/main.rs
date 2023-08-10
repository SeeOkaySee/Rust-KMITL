fn main() {
    println!("hello world");
}

fn count_negative(v: &[i64]) -> usize {
    let mut c = 0;
    for i in 0..v.len() {
        if v[i] < 0 { c += 1 }
    }
    c
}

fn doubles_loop(v: &mut [i64]) -> Vec<i64> {
    for i in 0..v.len() {
        v[i] *= 2;        
    }
    v.to_vec()
}

fn doubles_recursive(v: &mut [i64]) -> Vec<i64> {
    if !v.is_empty() {
        v[0] *= 2;
        doubles_recursive(&mut v[1..]);
    }
    v.to_vec()
}

#[test]
fn test_counting_0() {
    assert_eq!(count_negative(&[]), 0);
}

#[test]
fn test_counting_negative() {
    assert_eq!(count_negative(&[-1, 2, -3, 4, 5, 6]), 2);
}

#[test]
fn test_doubles_loop() {
    assert_eq!(doubles_loop(&mut [1, 2, 3, 4, 5, 6]), [2, 4, 6, 8, 10, 12]);
}

#[test]
fn test_doubles_loop2() {
    assert_eq!(doubles_loop(&mut [0, 20, 9]), [0, 40, 18]);
}

#[test]
fn test_doubles_recursion() {
    assert_eq!(doubles_recursive(&mut [1, 2, 3, 4, 5, 6]), [2, 4, 6, 8, 10, 12]);
}

#[test]
fn test_doubles_recursion2() {
    assert_eq!(doubles_recursive(&mut [0, 20, 9]), [0, 40, 18]);
}