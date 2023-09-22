use std::{env, f32};
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write, self, BufRead, BufReader};
use csv::{ReaderBuilder, Writer, WriterBuilder, Trim};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    polar: bool,
    cartesian: bool
}

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct PolarPoint {
    r: f32,
    t: f32
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("convert")
    .version("0.1.0")
    .about("converting points system")
    .arg(
        Arg::with_name("files")
            .value_name("FILE")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
    )
    .arg(
        Arg::with_name("polar")
            .short("p")
            .long("polar")
            .help("convert to polar")
            .takes_value(false)
            .conflicts_with("cartesian"),
    )
    .arg(
        Arg::with_name("cartesian")
            .short("c")
            .long("cartesian")
            .help("convert to cartesian")
            .takes_value(false),
    )
    .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        polar: matches.is_present("polar"),
        cartesian: matches.is_present("cartesian"),
    })
}

fn point_distance(x1: f32,y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}

fn to_polar(pt_list: Vec<Point>) -> Vec<PolarPoint> {
    let mut result = Vec::new();
    
    for point in pt_list {
        let r = point_distance(point.x,point.y,0.,0.);
        let y = point_distance(point.x,point.y,point.x,0.);
        let x = point_distance(point.x,0.,0.,0.);
        let angle = (x/y).atan();
        let to_degrees = angle * (180./3.14);
        
        result.push(PolarPoint{r:r, t:to_degrees})
    }
    result
}

fn to_cartesian(pt_list: Vec<PolarPoint>) -> Vec<Point> {
    let mut result = Vec::new();

    for point in pt_list {
        let x = point.r * point.t.cos();
        let y = point.r * point.t.sin();

        result.push(Point{x:x, y:y})
    }
    result
}

fn load_pairs_c<R: Read>(rdr: R) -> Vec<Point> {
    let mut reader
    = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);

    let mut out_list = vec![];
    for record in reader.records() {
        if let Ok(rec) = record {
            let v1: f32 = rec[0].parse().unwrap();
            let v2: f32 = rec[1].parse().unwrap();
            out_list.push(Point{x:v1,y:v2});
        }
    }
    out_list
}

fn load_pairs_p<R: Read>(rdr: R) -> Vec<PolarPoint> {
    let mut reader
    = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);

    let mut out_list = vec![];
    for record in reader.records() {
        if let Ok(rec) = record {
            let v1: f32 = rec[0].parse().unwrap();
            let v2: f32 = rec[1].parse().unwrap();
            out_list.push(PolarPoint{r:v1,t:v2});
        }
    }
    out_list
}

fn format_point_c(point: &Point) -> String {
    format!("\t<tr>\n\t\t<td>{:.1}</td>\n\t\t<td>{:.1}</td>\n\t</tr>", point.x, point.y)
}

fn format_point_p(point: &PolarPoint) -> String {
    format!("\t<tr>\n\t\t<td>{:.1}</td>\n\t\t<td>{:.3}</td>\n\t</tr>", point.r, point.t)
}

fn save_points_c<W: Write>(mut writer:Writer<W>, pt_list: Vec<Point>) -> Result<(), Box<dyn Error>> {
    let _ = writer.write_record(&["<style>\ntable, td { border: 1px solid #000000; border-collapse: collapse; }
        </style>\n<table>
        \t<tr>\n\t\t<th>Coordinate-X</th>\n\t\t<th>Coordinate-Y</th>\n\t</tr>"]);
    for point in pt_list {
        let formatted = format_point_c(&point);
        writer.write_record(&[formatted])?;
    }
    writer.flush()?;
    Ok(())
}

fn save_points_p<W: Write>(mut writer:Writer<W>, pt_list: Vec<PolarPoint>) -> Result<(), Box<dyn Error>> {
    let _ = writer.write_record(&["<style>\ntable, td { border: 1px solid #000000; border-collapse: collapse; }
        </style>\n<table>
        \t<tr>\n\t\t<th>distance</th>\n\t\t<th>angle</th>\n\t</tr>"]);
    for point in pt_list {
        let formatted = format_point_p(&point);
        writer.write_record(&[formatted])?;
    }
    writer.flush()?;
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(points) => {
                if config.polar {
                    let pointc = load_pairs_c(points);
                    let polar = to_polar(pointc);
                    let file = File::create("output_polar.html")?;
                    let path = env::current_dir();
                    let writer = WriterBuilder::new().delimiter(b',').from_writer(file);
                    save_points_p(writer, polar)?;
                    println!("Points saved to {}/output.html",path?.display());
                }
                else if config.cartesian {
                    let pointp = load_pairs_p(points);
                    let cartesian = to_cartesian(pointp);
                    let file = File::create("output_cartesian.html")?;
                    let path = env::current_dir();
                    let writer = WriterBuilder::new().delimiter(b',').from_writer(file);
                    save_points_c(writer, cartesian)?;
                    println!("Points saved to {}/output.html",path?.display());
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}