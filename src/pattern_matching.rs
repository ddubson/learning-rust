pub fn pattern_matching() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "Unknown", // Catch anything that's not matched to be unknown
        _ => "Invalid",       // Anything beyond 1 to 999 is Invalid, catch-all
    };

    println!("Country code matched to {}", country);

    let colors = 3;

    println!(
        "{}",
        match colors {
            1 => "Bright",
            2 | 3 => "Toned Down",                   // <= 2 or 3 matches
            _ if (colors % 3 == 0) => "alternating", // An if expression
            _ => "Dark",
        }
    );
}
