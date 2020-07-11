mod traits;
mod basic_types;
mod functional;
mod threading;
mod examples_io;
mod pattern_matching;
mod looping;
mod structs;
mod enums;

fn main() {
    basic_types::basic_types();
    traits::traits();
    functional::functions_as_data();
    pattern_matching::pattern_matching();
    looping::looping();
    structs::structs();
    enums::enums();
    threading::threads_and_channels();
    examples_io::guessing_game();
}
