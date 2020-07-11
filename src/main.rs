mod traits;
mod basic_types;
mod functional;
mod threading;
mod examples_io;
mod pattern_matching;
mod looping;

fn main() {
    basic_types::basic_types();
    traits::traits();
    functional::functions_as_data();
    threading::threads_and_channels();
    pattern_matching::pattern_matching();
    looping::looping();
    examples_io::guessing_game();
}
