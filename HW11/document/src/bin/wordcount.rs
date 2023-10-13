use std::fs;
use std::path::Path;
use std::env;

fn make_document(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(|s| s.trim().to_string())
        .collect()
}   

fn rank_documents<'a>(docs: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut temp_result: Vec<(String, usize)> = Vec::new();

    for doc in &docs {
        let count_paragraphs: Vec<&str> = doc.split("\n\n").collect();
        let count = count_paragraphs.len();

        temp_result.push((doc.clone(), count));
    }

    temp_result.sort_by(|a, b| b.1.cmp(&a.1));

    for (paragraph, _) in temp_result {
        result.push(paragraph);
    }

    result
}

fn count_words(doc: &Vec<String>) -> usize {
    doc.iter()
        .map(|paragraph| paragraph.split_whitespace().count())
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file1> <file2> <file3> ...", args[0]);
        std::process::exit(1);
    }

    // 2.1: Read files, count paragraphs, and generate an HTML table
    let files = &args[1..]; // Use command line arguments as file paths

    let mut documents: Vec<(String, Vec<String>)> = Vec::new();

    for file in files {
        let file_contents = fs::read_to_string(file).unwrap();
        let doc = make_document(&file_contents);
        documents.push((file.to_string(), doc))
    }

    documents.sort_by(|a, b| {
        let word_count_a = count_words(&a.1);
        let word_count_b = count_words(&b.1);
        word_count_b.cmp(&word_count_a)
    }); // Sort by paragraphs

    println!("HTML Table - Word Count:");
    let mut html = String::from("<style>\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>");

    html.push_str("<table>");
    html.push_str("\n\t<tr>\n\t<th>File</th>\n\t<th>Word Count</th>\n\t</tr>");
    for (filename,docu) in documents {
        html.push_str("\n\t<tr>");
        html.push_str(&format!("\n\t\t<td>{}</td>\n\t\t<td>{}</td>", filename, count_words(&docu)));
        html.push_str("\n\t</tr>");
    }
    html.push_str("</table>");
    println!("{}",html); // use cargo run --bin wordcount file1.txt file2.txt file3.txt > output1.html

}