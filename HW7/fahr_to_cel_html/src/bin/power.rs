
fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("<style>\ntable, td {{ border: 1px solid #000000; border-collapse: collapse; }} \n</style>");
    println!("");
    println!("<table>");
    println!("\t<tr>\n\t\t<th>x</th>\n\t\t<th>x^2</th>\n\t\t<th>x^3</th>\n\t</tr>");

    match args.len() {
        4 => {
            let mut start :i32 = args[1].parse().unwrap();
            let stop :i32 = args[2].parse().unwrap();
            let step :i32 = args[3].parse().unwrap(); 
            let n :i32 = (start + stop)/step;
            let d :i32 = (stop - start)/n;

            for _i in 0..n+1 {
                println!("\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",start, start.pow(2), start.pow(3));
                start += d;
            }
        }
        _ => {
            err()
        }
    }
    println!("</table>");
}

fn err() {
    println!("Please input sufficient data")
}