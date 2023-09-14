fn sort_ascending(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    list.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0) // Sort by x-coordinate
        } else {
            a.1.cmp(&b.1) // If x-coordinates are equal, sort by y-coordinate
        }
    });

    list
}

fn sort_descending(mut list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    list.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0) // Sort by x-coordinate in descending order
        } else {
            b.1.cmp(&a.1) // If x-coordinates are equal, sort by y-coordinate in descending order
        }
    });

    list
}

fn combine(list: Vec<i32>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    for i in (0..list.len() - 1).step_by(2) {
        result.push((list[i], list[i + 1]));
    }

    result
}

fn main() {
    let args: Vec<i32> = std::env::args()
        .skip(1) // Skip the first argument (Path)
        .filter_map(|arg| arg.parse().ok()) // Parse each argument as i32
        .collect();
    
    let combined = combine(args);
    let sorted_ascend = sort_ascending(combined.clone());
    let sorted_descend = sort_descending(combined.clone());
    println!("Sort by ascending: {:?}", sorted_ascend);
    println!("sort by descending: {:?}", sorted_descend);
}

#[test]
fn test_sort_ascending() {
    let mut input = vec![(3, 1), (4, 1), (5, 9), (6, 5), (3, 7)];
    let result = sort_ascending(input.clone());
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
fn test_sort_descending() {
    let mut input = vec![(3, 1), (4, 1), (5, 9), (6, 5), (3, 7)];
    let result = sort_descending(input.clone());
    input.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        } else {
            b.1.cmp(&a.1)
        }
    });
    assert_eq!(result, input);
}