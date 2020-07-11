fn for_loops() {
    for x in 1..11 {
        // for 1 through 10
        println!("x is {}", x);
    }

    for (index, x) in (30..41).enumerate() {
        println!("{}: {}", index, x);
    }
}

fn while_loops() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;
        println!("{}", x);
    }
}

fn infinite_loops() {
    let mut y = 1;

    loop {
        y *= 2;

        if y == 1024 {
            break;
        }
    }
}

pub fn looping() {
    for_loops();
    while_loops();
    infinite_loops();
}
