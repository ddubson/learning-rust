mod traits;
mod basic_types;
mod functional;
mod threading;
mod examples_io;

fn main() {
    basic_types::basic_types();
    traits::traits();
    functional::functions_as_data();
    threading::threads_and_channels();
    examples_io::guessing_game();
}
