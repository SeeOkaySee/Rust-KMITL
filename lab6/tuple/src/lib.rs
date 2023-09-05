fn pack_number_tuples(n: &[&i32], m: &[&i32]) -> Vec<(i32, i32)> {
    let nmax = n.len();
    let mmax = m.len();
    let mut max = 0;

    if nmax > mmax {
        max = nmax;
    } else {
        max = mmax;
    }

    if n.is_empty() && m.is_empty() {
        return Vec::new();
    } 
    else {
        let mut result: Vec<(i32, i32)> = Vec::new();
        for count in 0..max {
            let n_value = if count < n.len() { *n[count] } else { 0 };
            let m_value = if count < m.len() { *m[count] } else { 0 };
            result.push((n_value, m_value));
        }
        result
    }
}

fn pack_number_tuples_s(tuple1: &[i32],tuple2: &[i32]) -> Vec<(i32, i32)> {
    let mut iter1 = tuple1.iter();
    let mut iter2 = tuple2.iter();
    let mut result = Vec::new();

    while let (Some(&n1), Some(&n2)) = (iter1.next(), iter2.next()) {
        result.push((n1,n2))
    }
    result
}

#[test]
fn test_pack_number_tuples() {
assert_eq!(pack_number_tuples(&[], &[]), []);
assert_eq!(pack_number_tuples(&[&1], &[]), [(1, 0)]);
assert_eq!(pack_number_tuples(&[], &[&2, &3]), [(0, 2), (0, 3)]);
assert_eq!(
pack_number_tuples(&[&5, &1, &4], &[&2, &3]),
[(5, 2), (1, 3), (4, 0)]
);
}

#[test]
fn test_pack_number_tuples_s() {
assert_eq!(pack_number_tuples_s(&[], &[]), []);
assert_eq!(pack_number_tuples_s(&[1], &[]), []);
assert_eq!(pack_number_tuples_s(&[], &[2, 3]), []);
assert_eq!(
pack_number_tuples_s(&[5, 1, 4], &[2, 3]), [(5, 2), (1, 3)]
);
}
