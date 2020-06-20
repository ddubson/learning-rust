use std::io;
// Reference traits_examples file
mod traits_examples;

// Import specific Trait from module to be able to invoke it on a type
use traits_examples::Summary;

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

fn main() {
    println!("Hello World!");
    guessing_game();
    trait_example();
}
