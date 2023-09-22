use csv::{ReaderBuilder, Trim};
use std::io::Read;

struct Point {
    x: f32,
    y: f32,
    color : String,
}

fn point_distance(x1: f32,y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}

fn tag_points(pt_list: Vec<Point>) -> Vec<Point> {
    let mut tagged = Vec::new();
    let mut tagcolor = String::new();

    for point in pt_list {
        let distance = point_distance(point.x, point.y, 0.0,0.0);
        if distance <= 1. {
            tagcolor = "#80FF8080".to_string()
        } else {
            tagcolor = "#FF808080".to_string()
        }
        tagged.push(Point{
            x: point.x,
            y: point.y,
            color: tagcolor,
        })
    }
    tagged
}

fn load_points<R: Read>(r: R) -> Vec<Point> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(r);
    let mut tagged = Vec::new();

    for record in reader.records() {
        if let Ok(rec) = record {
            if rec.len() >= 2 {
                let x: f32 = rec[0].parse().unwrap();
                let y: f32 = rec[1].parse().unwrap();
                let color: String = "#000000".to_string();
                tagged.push(Point{x,y,color});
            }
        }
    }
    tagged   
}

fn main() {
    let points = vec![
        Point { x: 0.5, y: 0.5, color: String::new() },
        Point { x: 1.2, y: 0.0, color: String::new() },
        Point { x: -0.7, y: -0.7, color: String::new() },
        Point { x: -1.1, y: 4.0, color: String::new()}
    ];
    let data = "\
    1.0, 2.0\n\
    3.0, 4.0\n\
    ".as_bytes();

    let tagged_points = tag_points(points);
    let loaded_points = load_points(data);

    println!("---3.1---");
    for point in tagged_points {
        println!("({}, {}) {}", point.x, point.y, point.color);
    }

    println!("---3.2---");
    for point in loaded_points {
        println!("({}, {}) {}", point.x, point.y, point.color)
    }
}

#[test]
fn test_tag_points() {
    let points = vec![
        Point { x: 0.5, y: 0.5, color: String::new() },
        Point { x: 1.2, y: 0.0, color: String::new() },
        Point { x: -0.7, y: -0.7, color: String::new() },
        Point { x: -1.1, y: 4.0, color: String::new() },
    ];

    let tagged_points = tag_points(points);

    assert_eq!(tagged_points.len(), 4);
    assert_eq!(tagged_points[0].color, "#80FF8080");
    assert_eq!(tagged_points[1].color, "#FF808080");
    assert_eq!(tagged_points[2].color, "#80FF8080");
    assert_eq!(tagged_points[3].color, "#FF808080");
}

