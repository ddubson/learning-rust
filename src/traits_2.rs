trait Animal {
    // Static builder function that creates instances of Animal (self)
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) { // Traits can have default impls
        println!("{} cannot talk", self.name());
    }
}

#[derive(Debug)]
struct Human {
    name: &'static str
}

#[derive(Debug)]
struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

pub fn traits_2() {
    let h = Human { name: "John" };
    h.talk();

    // Use static create function
    let k = Human::create("Joe");
    println!("Trait: {:?}", k);

    // Use the trait function and statically infer which impl to use
    let j: Human = Animal::create("Jill");
    println!("Trait: {:?}", j);

    let c = Cat { name: "JJ" };
    c.talk(); // default impl.
}
