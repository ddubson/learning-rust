mod linked_list;

use crate::linked_list::linked_list_example;
use crate::mutability_copying_cloning::mutability_copying_cloning;
use crate::person::{Color, Person};

mod mutability_copying_cloning;
mod person;
mod point;

#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot Divide by Zero".to_string());
    }

    Res::Thing(a / b)
}

fn divide2(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot Divide by Zero".to_string());
    }

    Result::Ok(a / b)
}

pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        return Some(res);
    }
}

fn main() {
    let p: Person = Person {
        name: "John Doe".to_string(),
        age: 35,
        children: Some(4),
        fave_color: Some(Color::Green),
    };

    println!("Hello, {}", p.print());
    println!("Hello, {:?}", p);

    let c = Color::Red;

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    let a = divide(4, 5);
    let b = divide(10, 0);

    match a {
        Res::Thing(v) => println!("val = {}", v),
        _ => {}
    }

    if let Res::Thing(v) = a {
        println!("val = {}", v);
    }

    println!("a = {:?}, b = {:?}", a, b);

    let mut stepper = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };
    loop {
        match stepper.next() {
            Some(v) => println!("loop {}", v),
            None => break,
        }
    }

    mutability_copying_cloning();
    linked_list_example();
}
