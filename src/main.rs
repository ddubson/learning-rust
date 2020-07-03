use std::{io, thread};
// Reference traits_examples file
mod traits_examples;

// Import specific Trait from module to be able to invoke it on a type
use traits_examples::Summary;
use std::sync::mpsc;
use std::time::Duration;

fn guessing_game() {
    println!("Guess the number!");
    println!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess)
}

fn trait_example() {
    let tweet = traits_examples::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn functions_as_data() {
    let add = |x,y| x+y;

    println!("some of 2 + 2 is {}", add(2,2));
}

fn threads_and_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    println!("Hello World!");
    guessing_game();
    trait_example();
    functions_as_data();
    threads_and_channels();
}
