fn main() {
    println!("Hello, world!");
}

fn make_arrow(n: i32) -> String {
    let mut arrow = String::new();
    for i in 0..n {
        for _j in 0..i + 1 {
            arrow.push_str("*");
        }
        arrow.push_str("\n");
    }
    let m = n;
    for i in 0..n {
        for _j in 0..m - i - 1 {
            arrow.push_str("*");
        }
        arrow.push_str("\n");
    }
    return arrow;
}

pub fn make_arrow_recur (v: i32) -> String {
    let mut result: Vec<String> = Vec::new();
        let total_lines = 2*v-1;
        let mut arrow_str:String = String::new();
        for _i in 0..total_lines {
        if _i < v {
            for _j in 0.._i+1 {
                arrow_str.push('*');
            }
        } else {
            let num_asterisks = total_lines - _i;
            for _j in 0..num_asterisks {
                arrow_str.push('*');
            }
        } arrow_str.push('\n');
    } arrow_str
}

#[test]
fn test_make_arrow() {
    assert_eq!(make_arrow(3),"*\n**\n***\n**\n*\n\n");
    assert_eq!(make_arrow(4),"*\n**\n***\n****\n***\n**\n*\n\n");
    assert_eq!(make_arrow(5),"*\n**\n***\n****\n*****\n****\n***\n**\n*\n\n");
}

#[test]
fn test_make_arrow_recur() {
    assert_eq!(make_arrow_recur(3),"*\n**\n***\n**\n*\n");
    assert_eq!(make_arrow_recur(4),"*\n**\n***\n****\n***\n**\n*\n");
    assert_eq!(make_arrow_recur(5),"*\n**\n***\n****\n*****\n****\n***\n**\n*\n");
}