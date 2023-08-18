fn extract_non_negatives(v: &[f32]) -> Vec<f32> {
    let mut result = Vec::new();
    for i in v {
        if i >= &0. {
            result.push(*i);
        }
    }
    result
}

fn extract_negatives(v: &[f32]) -> Vec<f32> {
    let mut n_result = Vec::new();
    for i in v {
        if i < &0. {
            n_result.push(*i);
        }
    }
    n_result
}

fn extract_non_negatives_reccursion(v: &[f32], i: usize) -> Vec<f32> {
    if i >= v.len() {
        return Vec::new();
    }
    let mut result = extract_non_negatives_reccursion(v, i + 1);

    if v[i] >= 0. {
        result.push(v[i]);
    }
    result
}

fn extract_non_negatives_rhandle(v: &[f32]) -> Vec<f32> {
    extract_non_negatives_reccursion(v, 0)
}

fn split_non_negatives(v: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let non_negatives = extract_non_negatives(v);
    let negatives = extract_negatives(v);
    (non_negatives, negatives)
}

#[test]
fn test_extract_non_negatives() {
assert_eq!(extract_non_negatives(&[]), &[]);
assert_eq!(
extract_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
[0.8, 1.6, 10.5]
);
assert_eq!(extract_non_negatives(&[7.8, 98234.01, -5012.77, -5.4, 2.0]), &[7.8, 98234.01, 2.0]);
}

#[test]
fn test_extract_negatives() {
assert_eq!(extract_negatives(&[]), &[]);
assert_eq!(
extract_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
[-5.1, -6.5]
);
}

#[test]
fn test_extract_non_negatives_recur() {
assert_eq!(extract_non_negatives_rhandle(&[]), &[]);
assert_eq!(
extract_non_negatives_rhandle(&[0.8, -5.1, 1.6, -6.5, 10.5]),
[10.5, 1.6, 0.8]
);
assert_eq!(extract_non_negatives_rhandle(&[7.8, 98234.01, -5012.77, -5.4, 2.0]), &[2.0, 98234.01, 7.8]);
}

#[test]
fn test_split_non_negatives() {
assert_eq!(split_non_negatives(&[]), ([].to_vec(), [].to_vec()));
assert_eq!(
split_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
(
[0.8, 1.6, 10.5].to_vec(),
[-5.1, -6.5].to_vec()
)
);
}