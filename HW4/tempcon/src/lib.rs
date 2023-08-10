fn _fahr_to_cel_v(v: &mut [f32]) -> Vec<f32> {
    for i in 0..v.len() {
        //replace farenhiet with celcius value in each index
        v[i] = (v[i] - 32.) * 5./9.
    }
    v.to_vec()
}

fn _fahr_to_cel_recur(v: &mut [f32]) -> Vec<f32> {
    if v.is_empty() {
        //returns new vector if there's no slice
        return Vec::new();
    }
    let mut cel = vec![(v[0] - 32.0) * 5.0 / 9.0];
    cel.extend_from_slice(&_fahr_to_cel_v(&mut v[1..]));
    cel
}

#[test]
fn test_fahr_to_cel_v() {
    assert_eq!(_fahr_to_cel_v(&mut[300.,250.,200.,150.,100.]), [148.88889, 121.111115, 93.333336, 65.55556, 37.77778]);
    assert_eq!(_fahr_to_cel_v(&mut[32.,64.,128.]), [0.0, 17.777779, 53.333332]);
    assert_eq!(_fahr_to_cel_v(&mut[0.,1.,2.,3.,4.,5.,1000.]), [-17.777779, -17.222221, -16.666666, -16.11111, -15.555555, -15.0, 537.7778]);
}

#[test]
fn test_fahr_to_cel_recur() {
    assert_eq!(_fahr_to_cel_recur(&mut[300.,250.,200.,150.,100.]), [148.88889, 121.111115, 93.333336, 65.55556, 37.77778]);
    assert_eq!(_fahr_to_cel_recur(&mut[32.,64.,128.]), [0.0, 17.777779, 53.333332]);
    assert_eq!(_fahr_to_cel_recur(&mut[0.,1.,2.,3.,4.,5.,1000.]), [-17.777779, -17.222221, -16.666666, -16.11111, -15.555555, -15.0, 537.7778]);
}