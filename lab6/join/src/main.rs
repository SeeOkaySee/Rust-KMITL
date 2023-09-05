fn join_strings(words: Vec<&str>,sep: &str) {

    for word in words {
        print!("{}{}",word,sep);
    }
}

fn main() {
    join_strings(vec!["C","Rust","C++","Python"],",");
}
