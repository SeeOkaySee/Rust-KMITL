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

fn make_arrow2(n: i32) -> String {
    let mut arrow = String::new();
    for i in 1..n {
        for _j in 1..i {
            arrow.push_str(" ") 
        }
        for _l in 0..i {
            arrow.push_str("*")
        } 
        arrow.push_str("\n")
    }
    
    for i in 0..n {
        for _j in 0..i {
            arrow.push_str(" ")
        }
        for _k in i..n {
            arrow.push_str("*")
        }

        arrow.push_str("\n")
    }
    return arrow;
}

fn make_arrow_recursive(v: i32, total_lines: i32, i: i32) -> String {
    if i == total_lines {
        return String::new();
    }

    let mut arrow_str = String::new();
    if i < v {
        for _j in 0..i + 1 {
            arrow_str.push('*');
        }
    } else {
        let num_asterisks = total_lines - i;
        for _j in 0..num_asterisks {
            arrow_str.push('*');
        }
    }
    arrow_str.push('\n');

    arrow_str.push_str(&make_arrow_recursive(v, total_lines, i + 1));
    arrow_str
}

fn make_arrow_recur(v: i32) -> String {
    let total_lines = 2 * v - 1;
    make_arrow_recursive(v, total_lines, 0)
}

fn make_arrow2_recur(n: i32) -> String {
    fn build_arrow_rec(i: i32, n: i32) -> String {
        if i == n {
            return String::new();
        }

        let mut line = String::new();

        for _j in 0..i {
            line.push_str(" ");
        }

        for _k in i..n {
            line.push_str("*");
        }

        line.push_str("\n");

        let next_line = build_arrow_rec(i + 1, n);

        line + &next_line
    }
    fn build_arrow_rec2(i: i32, n: i32) -> String {
        if i == n {
            return String::new();
        }

        let mut line = String::new();

        for _j in 1..i {
            line.push_str(" ");
        }

        for _k in 0..i {
            line.push_str("*");
        }

        line.push_str("\n");

        let next_line = build_arrow_rec2(i + 1, n);

        line + &next_line
    }

    let top = build_arrow_rec2(1, n);
    let bottom = build_arrow_rec(0, n);

    top + &bottom
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

#[test]
fn test_make_arrow2() {
    assert_eq!(make_arrow2(3),"*\n **\n***\n **\n  *\n");
    assert_eq!(make_arrow2(4),"*\n **\n  ***\n****\n ***\n  **\n   *\n");
    assert_eq!(make_arrow2(5),"*\n **\n  ***\n   ****\n*****\n ****\n  ***\n   **\n    *\n");
}

#[test]
fn test_make_arrow2_recur() {
    assert_eq!(make_arrow2_recur(3),"*\n **\n***\n **\n  *\n");
    assert_eq!(make_arrow2_recur(4),"*\n **\n  ***\n****\n ***\n  **\n   *\n");
    assert_eq!(make_arrow2_recur(5),"*\n **\n  ***\n   ****\n*****\n ****\n  ***\n   **\n    *\n");
}