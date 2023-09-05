fn min_max_avg(arr: &[i32]) -> Vec<(i32,i32,f32)> {
    let mut result = Vec::new();
    let mut iter = arr.iter();
    let mut sum = 0.;
    let (mut min, mut max) = (arr[0],arr[0]);

    while let Some(num) = iter.next(){
        sum += *num as f32;
        if min > *num {
            min = *num;
        }
        if max < *num {
            max = *num;
        }
    }
    let av = sum/(arr.len() as f32 + 1.);
    result.push((min,max,av));
    result
}

fn partial_sum(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut iter = arr.iter();
    
    if let Some(&first) = iter.next() {
        result.push(first);

        for val in iter {
            result.push(result.last().unwrap() + val);
        }
    }

    result
}

#[test]
fn test_min_max_avg() {
    assert_eq!(min_max_avg(&[2,72,7,98,36,300]), vec![(2, 300, 73.57143)]);
}

#[test]
fn test_partial_sum() {
    assert_eq!(partial_sum(&[1, 3, 5, 7, 100]), vec![1, 4, 9, 16, 116]);
}