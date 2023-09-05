fn pack_number_tuples3(val1: &[i32], val2: &[i32], val3: &[i32]) -> Vec<(i32, i32, i32)> {
    let max_len = std::cmp::max(val1.len(), std::cmp::max(val2.len(), val3.len()));
    
    let result: Vec<(i32, i32, i32)> = (0..max_len)
        .map(|i| {
            let x = val1.get(i).cloned().unwrap_or(0);
            let y = val2.get(i).cloned().unwrap_or(0);
            let z = val3.get(i).cloned().unwrap_or(0);
            (x, y, z)
        })
        .collect();

    result
}

fn pack_number_tuples_s3(val1: &[i32], val2: &[i32], val3: &[i32]) -> Vec<(i32, i32, i32)> {
    let min_len = std::cmp::min(val1.len(), std::cmp::min(val2.len(), val3.len()));
    
    let result: Vec<(i32, i32, i32)> = (0..min_len)
        .map(|i| (val1[i], val2[i], val3[i]))
        .collect();

    result
}

#[test]
fn test_pack_number_tuples3() {
    assert_eq!(pack_number_tuples3(&[1, 2], &[4, 3], &[5]), vec![(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuples3(&[-7,3], &[-9,-5], &[7,7]), vec![(-7, -9, 7), (3, -5, 7)]);
    assert_eq!(pack_number_tuples3(&[], &[], &[]), vec![])
}

#[test]
fn test_pack_number_tuples_s3() {
    assert_eq!(pack_number_tuples_s3(&[1, 2], &[4, 3], &[5]), vec![(1, 4, 5)]);
    assert_eq!(pack_number_tuples_s3(&[-7, 3], &[-9, 3], &[3]), vec![(-7, -9, 3)]);
    assert_eq!(pack_number_tuples_s3(&[], &[], &[]), vec![])
}


