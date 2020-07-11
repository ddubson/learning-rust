mod basic_types;
mod enums;
mod examples_io;
mod functional;
mod lifetimes;
mod looping;
mod pattern_matching;
mod structs;
mod threading;
mod traits;
mod traits_2;

fn main() {
    basic_types::basic_types();
    functional::functions_as_data();
    pattern_matching::pattern_matching();
    looping::looping();
    structs::structs();
    enums::enums();
    traits::traits();
    traits_2::traits_2();
    lifetimes::lifetimes();
    traits_2::traits_to_external_structs();
    threading::threads_and_channels();
    examples_io::guessing_game();
}
