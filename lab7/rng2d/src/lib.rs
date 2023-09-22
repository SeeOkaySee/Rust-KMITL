use rand::Rng;

fn filter_points(points: &[(f32,f32)]) -> Vec<(f32,f32)> {
    let mut result = Vec::new();
    let mut iter = points.iter();

    while let Some((x,y)) = iter.next() {
        if (x+y).sqrt() <= 1. {
            result.push((*x,*y));
        }
    }
    result
}

fn gen_points<R: Rng>(rng: &mut R,n: i32) -> Vec<(f32,f32)> {
    let mut result = Vec::new();

    for _i in 1..=n {
        result.push((rng.gen_range(-1. ..= 1.),rng.gen_range(-1. ..= 1.)));
    }
    result
}

#[test]
fn test_filter_points() {
    assert_eq!(filter_points(&[]), Vec::new());
    assert_eq!(filter_points(&[(2.0, 3.0), (5.0, 6.0)]), Vec::new());
    assert_eq!(filter_points(&[(-2.0, -2.0), (-1.5, 1.5), (0.0, 0.0), (1.5, -1.5), (2.0, 2.0)]), vec![(-1.5, 1.5), (0.0, 0.0), (1.5, -1.5)]);
    assert_eq!(filter_points(&[(-2.0, -2.0), (-1.5, -1.5), (-1.0, -1.0), (-0.5, -0.5), (0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (1.5, 1.5), (2.0, 2.0)])
    ,vec![(0.0, 0.0), (0.5, 0.5)]);
}

#[test]
fn test_gen_points() {
    let mut rng = rand::thread_rng();

    let generated_empty = gen_points(&mut rng, 0);
    assert_eq!(generated_empty.len(), 0);

    let generated_positive = gen_points(&mut rng, 5);
    assert_eq!(generated_positive.len(), 5);
    for &(x, y) in &generated_positive {
        assert!(x >= -1.0 && x <= 1.0);
        assert!(y >= -1.0 && y <= 1.0);
    }

    let generated_negative = gen_points(&mut rng, -5);
    assert_eq!(generated_negative.len(), 0);
}