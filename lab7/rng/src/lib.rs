use rand::Rng;

fn filter_numbers(list: &[f32]) -> Vec<f32> {
    let mut result = Vec::new();
    let mut iter = list.iter();

    while let Some(number) = iter.next() {
        if &-1.0 <= number && number <= &1.0 {
            result.push(*number);
        }
        else {
            continue
        }
    }
    result
}

fn gen_number<R: Rng>(rng: &mut R,times: i32) -> Vec<f32> {
    let mut result = Vec::new();

    for _i in 1..=times {
        result.push(rng.gen_range(-1. ..= 1.));
    }
    result
}

#[test]
fn test_filter_numbers() {
    assert_eq!(filter_numbers(&[]), ([]));
    assert_eq!(filter_numbers(&[2.0, 0.4, 5.0, -1.0, 0.8]), ([0.4, -1.0, 0.8]));
}

#[test]
fn test_gen_number() {
    let mut rng = rand::thread_rng();

    let generated_empty = gen_number(&mut rng, 0);
    assert_eq!(generated_empty.len(), 0);

    let generated_positive = gen_number(&mut rng, 5);
    assert_eq!(generated_positive.len(), 5);
    for &num in &generated_positive {
        assert!(num >= -10.0 && num <= 10.0);
    }

    let generated_negative = gen_number(&mut rng, -5);
    assert_eq!(generated_negative.len(), 0);
}