
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        4 => {
            let mut start :i32 = args[1].parse().unwrap();
            let stop :i32 = args[2].parse().unwrap();
            let step :i32 = args[3].parse().unwrap(); 
            let n :i32 = (start + stop)/step;
            let d :i32 = (stop - start)/n;

            println!("Fahr\tCelcius");  
            for _i in 0..n+1 {
                let c:f32 = temptable::conv(start);
                println!("{:>4}\t{:>4.1}",start as f32, c);
                start += d;
            }
        }
        _ => {
            err()
        }
    }
}

fn err() {
    println!("Please input sufficient data")
}