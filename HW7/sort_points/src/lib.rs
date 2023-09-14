fn bubble_sort_ascending(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut n = list.len();
    let mut swapped = true;
    
    while swapped {
        swapped = false;
        for i in 1..n {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }

    list
}

fn bubble_sort_descending(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut n = list.len();
    let mut swapped = true;
    
    while swapped {
        swapped = false;
        for i in 1..n {
            if list[i - 1] < list[i] {
                list.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }

    list
}

#[test]
fn test_bubble_sort_ascending() {
    let mut input = vec![(3, 1), (4, 1), (5, 9), (6, 5), (3, 7)];
    let result = bubble_sort_ascending(input.clone());
    input.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    assert_eq!(result, input);
}

#[test]
fn test_bubble_sort_descending() {
    let mut input = vec![(3, 1), (4, 1), (5, 9), (6, 5), (3, 7)];
    let result = bubble_sort_descending(input.clone());
    input.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        } else {
            b.1.cmp(&a.1)
        }
    });
    assert_eq!(result, input);
}
