use std::env;

fn main() {
    // parsing
    let args: Vec<String> = env::args().collect();
    let g_args = if args.len() < 2 { "" } else { &args[1] };
    let g: i32 = g_args.parse().unwrap();

    // conditional
    if g >= 95 && g < 101{
        println!("Excellent with A+");
    }
    else if g >= 81 && g < 95{
        println!("A");
    }
    else if g >= 71 && g < 81{
        println!("B");
    }
    else if g >= 61 && g < 71{
        println!("C");
    }
    else if g >= 50 && g < 61{
        println!("D");
    }
    else if g >= 0 && g < 50{
        println!("Failed with F");
    }
    else{
        println!("Invalid score");
    }
    
}
