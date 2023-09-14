fn sort_descending(mut list: Vec<i32>) -> Vec<i32> {
    list.sort_by(|a, b| b.cmp(a));
    list
}

fn sort_ascending(mut list: Vec<i32>) -> Vec<i32> {
    list.sort_by(|a, b| a.cmp(b));
    list
}

fn main() {
    let args: Vec<i32> = std::env::args()
        .skip(1) // Skip the first argument which is the program name
        .filter_map(|arg| arg.parse().ok()) // Parse each argument as i32
        .collect();

    let descending_list = sort_descending(args.clone());
    let ascending_list = sort_ascending(args.clone());
    println!("Sort by descending: {:?}", descending_list);
    println!("Sort by ascending: {:?}", ascending_list);
}

#[test]
fn run_asc() {
    assert_eq!(sort_ascending(vec![]), vec![]);
    assert_eq!(sort_ascending(vec![3, 1, 4, 1, 5, 9, 6, 5, 3, 5]), vec![1, 1, 3, 3, 4, 5, 5, 5, 6, 9]);
    assert_eq!(sort_ascending(vec![3, 8, 7, 4, 5]), vec![3, 4, 5, 7, 8]);
}

#[test]
fn run_desc() {
    assert_eq!(sort_descending(vec![]), vec![]);
    assert_eq!(sort_descending(vec![3, 1, 4, 1, 5, 9, 6, 5, 3, 5]), vec![9, 6, 5, 5, 5, 4, 3, 3, 1, 1]);
    assert_eq!(sort_descending(vec![3, 8, 7, 4, 5]), vec![8, 7, 5, 4, 3]);
}