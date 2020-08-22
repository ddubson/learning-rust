pub fn str_vs_string_example() {
    // s is a pointer on the stack
    let s = "    hello     ";
    let p = s.trim();
    println!("p == {}", p);

    // s is now on the heap
    let mut s = "    hello     ".to_string();
    let p = s.trim();
    println!("p == {}", p);
    s.push_str("goodbye");
}