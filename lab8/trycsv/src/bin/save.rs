use csv::{Writer,WriterBuilder};
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::env;
use tempfile::tempfile;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
    color : String,
}

fn point_distance(x1: f32,y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}

fn format_point(point: &Point) -> String {
    format!("{:.1}, {:.1}, {}", point.x, point.y, point.color)
}

fn save_points<W: Write>(mut writer:Writer<W>, pt_list: Vec<Point>) -> Result<(), Box<dyn Error>> {
    for point in pt_list {
        let formatted = format_point(&point);
        writer.write_record(&[formatted])?;
    }
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let points = vec![
        Point {
            x: 4.0,
            y: 2.0,
            color: "#FF808080".to_string(),
        },
        Point {
            x: 0.3,
            y: 0.5,
            color: "#80FF8080".to_string(),
        },
        Point {
            x: 0.5,
            y: 0.5,
            color: "#80FF8080".to_string(),
        },
        Point {
            x: 12.0,
            y: 0.0,
            color: "#FF808080".to_string(),
        }
    ];
    
    let path = env::current_dir();
    let file = File::create("output.csv")?;
    let mut writer = WriterBuilder::new().delimiter(b',').from_writer(file);
    save_points(writer, points)?;
    println!("Points saved to {}/output.csv",path?.display());
    Ok(())
}

#[test]
fn test_save_points() {
    let points = vec![
        Point {
            x: 1.0,
            y: 2.0,
            color: "#FF808080".to_string(),
        },
        Point {
            x: 3.0,
            y: 4.0,
            color: "#80FF8080".to_string(),
        },
    ];

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_owned();
    let file = File::create(&file_path).unwrap();

    let mut writer = WriterBuilder::new().delimiter(b',').from_writer(file);
    save_points(writer, points.clone()).unwrap();

    let file_contents = std::fs::read_to_string(&file_path).unwrap();
    let expected_contents = "\"1.0, 2.0, #FF808080\"\n\"3.0, 4.0, #80FF8080\"\n";
    assert_eq!(file_contents, expected_contents);
}