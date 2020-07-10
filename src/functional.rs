pub fn functions_as_data() {
    let add = |x,y| x+y;

    println!("some of 2 + 2 is {}", add(2,2));
}