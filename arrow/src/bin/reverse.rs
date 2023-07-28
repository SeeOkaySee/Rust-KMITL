
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size_arg = if args.len() < 2 {""} else {&args[1]};
    let n: i32 = size_arg.parse().unwrap_or(0);
    
    for _i in (1..n).rev() {
            for _j in 1..=_i {
                print!(" ") 
            }
            for _l in _i..n {
                print!("*")
            } println!()
        }
        for _i in 0..n {
            for _j in 0.._i {
                print!(" ")
            }
            for _k in _i..n {
                print!("*")
            } println!()
        }
}