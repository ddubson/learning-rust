struct Point {
    x: f64,
    y: f64
}

pub fn structs() {
    let p = Point { x: 2.0, y: 4.0 };
    println!("Struct point: {}, {}", p.x, p.y);
}