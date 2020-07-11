// Types: https://doc.rust-lang.org/book/ch03-02-data-types.html

use std::mem::size_of_val;

pub fn basic_types() {
    let a: i8 = 127; // i8 is a signed byte, range from -128 to 127

    println!("a is a signed byte ({} bytes)", size_of_val(&a));

    let b: u8 = 255; // u8 is an unsigned byte, range from 0 to 255

    println!("b is an unsigned byte ({} bytes)", size_of_val(&b));

    let c: i32 = 789; // i32 is a signed 32-bit value

    println!("c is a signed 32-bit value ({} bytes)", size_of_val(&c));

    let d: isize = 2000; // isize is a signed WORD value (based on the machine)

    println!("d is a signed word value ({} bytes)", size_of_val(&d));

    let e: usize = 20000; // usize is an unsigned WORD value (based on the machine)

    println!("e is an unsigned word value ({} bytes)", size_of_val(&e));
}
