// 3.2
use std::f64::consts::PI;

trait Shape: ShapeClone {
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
}

#[derive(Debug, Clone, Copy)]
enum ShapeEnum {
    Circle(i32, i32, i32),
    Rectangle(i32, i32, i32, i32),
    Triangle((i32,i32) , (i32,i32) , (i32,i32)),
}

impl ShapeEnum {
    fn rep_string(&self) -> String {
        match self {
            ShapeEnum::Circle(x, y, r) => format!("<Circle: {}, {}, {}>", x, y, r),
            ShapeEnum::Rectangle(x, y, w, h) => format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h),
            ShapeEnum::Triangle(co1,co2,co3) => format!("<Triangle: {:?}, {:?}, {:?}", co1, co2, co3),
        }
    }

    fn area(&self) -> f64 {
        match self {
            ShapeEnum::Circle(_, _, r) => PI * (*r * *r) as f64,
            ShapeEnum::Rectangle(_, _, w, h) => (*w * *h) as f64,
            ShapeEnum::Triangle((x1,y1),(x2,y2),(x3,y3)) => 0.5 * ((x1-x3)*(y2-y1) - (x1-x2)*(y3-y1)) as f64,
        }
    }
}

#[derive(Debug, Clone)]
struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

#[derive(Debug, Clone)]
struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Debug, Clone)]
struct Triangle {
    co1: (i32, i32),
    co2: (i32, i32),
    co3: (i32, i32),
}

impl Shape for Circle {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.r)
    }

    fn area(&self) -> f64 {
        PI * (self.r * self.r) as f64
    }
}

impl Shape for Rectangle {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.w, self.h)
    }

    fn area(&self) -> f64 {
        (self.w * self.h) as f64
    }
}

impl Shape for Triangle {
    fn rep_string(&self) -> String {
        format!("<Triangle: {:?}, {:?}, {:?}", self.co1, self.co2, self.co3)
    }
    
    fn area(&self) -> f64 {
        0.5 * ((self.co1.0-self.co3.0)*(self.co2.1-self.co1.1) - (self.co1.0-self.co2.0)*(self.co3.1-self.co1.1)) as f64
    }
}

impl Circle {
    fn new(x: i32, y: i32, r: i32) -> Box<dyn Shape> {
        Box::new(Circle { x, y, r })
    }
}

impl Rectangle {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Box<dyn Shape> {
        Box::new(Rectangle { x, y, w, h })
    }
}

impl Triangle {
    fn new(co1: (i32,i32), co2: (i32,i32), co3: (i32,i32)) -> Box<dyn Shape> {
        Box::new(Triangle { co1, co2, co3 })
    }
}

fn input_shape_list() -> Vec<Box<dyn Shape>> {
    vec![
        Circle::new(0, 0, 1),
        Circle::new(50, 50, 15),
        Rectangle::new(40, 40, 20, 20),
        Rectangle::new(10, 40, 15, 10),
    ]
}

trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_box()
    }
}

impl<T> ShapeClone for T
where
    T: 'static + Shape + Clone,
{
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(
        output,
        &[
            "<Circle: 0, 0, 1>",
            "<Circle: 50, 50, 15>",
            "<Rectangle: 40, 40, 20, 20>",
            "<Rectangle: 10, 40, 15, 10>"
        ]
    );
}

#[test]
fn test_shapes_002() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(
        output,
        &[
            "<Circle: 0, 0, 1>, area: 3.14",
            "<Circle: 50, 50, 15>, area: 706.86",
            "<Rectangle: 40, 40, 20, 20>, area: 400.00",
            "<Rectangle: 10, 40, 15, 10>, area: 150.00"
        ]
    );
}

#[test]
fn test_shapes_003() {
    let input_list = input_shape_list();
    let shape_list = input_list.iter().cloned().map(|s| s.clone_box());
    let omap = shape_list.map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(
        output,
        &[
            "<Circle: 0, 0, 1>, area: 3.14",
            "<Circle: 50, 50, 15>, area: 706.86",
            "<Rectangle: 40, 40, 20, 20>, area: 400.00",
            "<Rectangle: 10, 40, 15, 10>, area: 150.00"
        ]
    );
}

fn main() {
    test_shapes_001();
    test_shapes_002();
    test_shapes_003();
}