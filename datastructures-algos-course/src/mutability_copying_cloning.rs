use crate::person::Person;
use crate::point::Point;

pub fn mutability_copying_cloning() {
    let mut x = 34;
    let y = x;
    x += 5;
    // i32 implements the Copy trait, hence we can use both x and y
    println!("y = {}, x = {}", y, x);

    let mut p = Person {
        name: "Jill".to_string(),
        age: 22,
        children: None,
        fave_color: None,
    };

    // ❌❌❌ ERROR! p2 has borrowed p, but we're trying to access it (use of borrowed value)
    //let p2 = p;
    //println!("p = {:?}, p2 = {:?}", p, p2);

    // ✅✅✅
    let p2 = p.clone();
    p.name.push_str(" Doe");
    println!("p = {:?}, p2 = {:?}", p, p2);

    // ----

    // Point implements the Copy trait, so cloning is implicit for a struct that has primitives
    let point = Point::new(3, 4);
    let point2 = point;
    println!("point = {:?}, point2 = {:?}", point, point2);
}
