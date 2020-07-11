#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

// Define a lifetime called 'z'
#[derive(Debug)]
struct Company<'z> {
    name: String,
    ceo: &'z Person, // the reference to person lives as long as Company, same lifetime
}

pub fn lifetimes() {
    // &'static str <- 'static' is a lifetime - how long the variable will live, static lives for the whole program

    let boss = Person {
        name: String::from("Jill Doe"),
    };
    let comp = Company {
        name: String::from("Company"),
        ceo: &boss,
    };

    println!("{:?}", comp)
}
