#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub children: Option<i32>,
    pub fave_color: Option<Color>,
}

impl Person {
    pub fn print(&self) -> String {
        format!(
            "name = {}, age = {}, children = {}",
            self.name,
            self.age,
            self.children.unwrap_or(0)
        )
    }

    pub fn age_up(&mut self) {
        self.age += 1
    }
}

pub fn get_age(person: &Person) -> &i32 {
    &person.age
}
