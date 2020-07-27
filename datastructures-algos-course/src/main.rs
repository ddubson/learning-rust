#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

impl Person {
    pub fn print(&self) -> String {
        format!(
            "name = {}, age = {}, children = {}",
            self.name, self.age, self.children
        )
    }
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let p: Person = Person {
        name: "John Doe".to_string(),
        age: 35,
        children: 4,
        fave_color: Color::Green,
    };

    println!("Hello, {}", p.print());
    println!("Hello, {:?}", p);

    let c = Color::Red;

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
