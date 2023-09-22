use rand::Rng;

fn probability<R: Rng>(rng: &mut R,n: usize) -> f32 {
    let mut val10 = Vec::new();

    // generate n values and put it in a Vector.
    for _i in 1..=n {
        val10.push(rng.gen_range(-10. ..= 10.));
    }
    
    let mut iter = val10.iter();
    let mut count = 0;
    
    // iter through the vector and check whether the generated value is in the range.
    while let Some(val) = iter.next() {
        if &-1.0 <= val && val <= &1.0 {
            count += 1;
        }
    }
    let probability = count as f32/n as f32;
    probability as f32
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_args = if args.len() < 2 { "" } else { &args[1] };
    let n: usize = match n_args.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("input a valid integer argument.");
            return;
        }
    };

    let mut rng = rand::thread_rng();
    println!("{}", probability(&mut rng, n));
}
