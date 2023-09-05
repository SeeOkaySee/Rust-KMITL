fn unpack_number_tuples(tuples: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut first = Vec::new();
    let mut second = Vec::new();

    for &(n1, n2) in tuples {
        first.push(n1);
        second.push(n2);
    }

    (first, second)
}

fn unpack_number_tuples3(tuples: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    let mut third_numbers = Vec::new();

    for &(n1, n2, n3) in tuples {
        first_numbers.push(n1);
        second_numbers.push(n2);
        third_numbers.push(n3);
    }

    (first_numbers, second_numbers, third_numbers)
}

#[test]
fn test_unpack_number_tuples(){
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (3, 2), (2,1)]),  (vec![1, 3, 2], vec![4, 2, 1]));
    assert_eq!(unpack_number_tuples(&[(-1, -4), (-2, -5), (-3, -6)]), (vec![-1, -2, -3], vec![-4, -5, -6]));
}

#[test]
fn test_unpack_number_tuples3(){
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1)]),  (vec![1, 2], vec![4, 2], vec![5,1]));
    assert_eq!(unpack_number_tuples3(&[(-1, -4, -2), (-5, -3, -6)]), (vec![-1, -5], vec![-4, -3], vec![-2, -6]));
}
