use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    name: Rc<String>,
}

struct ArcPerson {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

pub fn reference_counter() {
    let name = Rc::new("Jill".to_string());
    let person = Person::new(name.clone()); // when you move an Rc, clone it!
    person.greet();
    println!("RC:: Name = {}", *name);
}

pub fn atomic_reference_counter() {
    let name = Arc::new("Jill".to_string());
    let state = Arc::new(Mutex::new(String::from("bored")));
    let person = ArcPerson::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!(
        "ARC:: Name = {}, State = {}",
        *name,
        state.lock().unwrap().as_str()
    );
    t.join().unwrap();
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

impl ArcPerson {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> ArcPerson {
        ArcPerson { name, state }
    }

    fn greet(&self) {
        // Lock the mutex
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("Excited");

        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}
