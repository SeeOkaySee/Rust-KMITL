use std::env;

fn main() {
    // Collect value
    let args: Vec<String> = env::args().collect();
    
    // Check args.len
    let n_args = if args.len() < 2 { "" } else { &args[1] };

    // Parsing args
    let n: i32 = match n_args.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("input a valid integer argument.");
            return;
        }
    };
    let m = n;

    for i in 0..n {
        for _j in 0..i + 1 {
            print!("*");
        }
        println!("");
    }

    for i in 0..n {
        for _j in 0..m - i - 1 {
            print!("*");
        }
        println!("");
    }
}
