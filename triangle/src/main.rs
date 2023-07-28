use std::env;

fn main() {
    //collect value
    let args: Vec<String> = env::args().collect();
    //check args.len
    let n_args = if args.len() < 2 {""} else {&args[1]};
    //parsing args
    let n: i32 = n_args.parse().unwrap();

    for i in 0..n {
        for j in 0..i + 1 {
            print!("*");
        }
        println!("");
    }
}
