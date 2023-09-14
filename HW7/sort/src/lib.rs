fn bubble_sort_descending(list: &mut [i32]) -> &[i32] {
    let len = list.len();

    for i in 0..len {
        for j in 0..len - 1 {
            if list[j] < list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
    list
}

fn bubble_sort_ascending(list: &mut [i32]) -> &[i32] {
    let len = list.len();

    for i in 0..len {
        for j in 0..len - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
    list
}

#[test]
fn test_bubble_sort_ascending() {
    assert_eq!(bubble_sort_ascending(&mut []), []);
    let mut input = vec![3, 1, 4, 1, 5, 9, 6, 5, 3, 5];
    let result = bubble_sort_ascending(&mut input);
    assert_eq!(result, [1, 1, 3, 3, 4, 5, 5, 5, 6, 9]);
}

#[test]
fn test_bubble_sort_descending() {
    assert_eq!(bubble_sort_ascending(&mut []), []);
    let mut input = vec![3, 1, 4, 1, 5, 9, 6, 5, 3, 5];
    let result = bubble_sort_descending(&mut input);
    assert_eq!(result, [9, 6, 5, 5, 5, 4, 3, 3, 1, 1]);
}

