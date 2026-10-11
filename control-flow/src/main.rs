fn main() {
    let mut i = 32;
    
    // If elseif else
    if (i < 0) {
        println!("i is less than 0");
    } else if (i == 32) {
        println!("i is 32");
    } else {
        println!("i is something else");
    }
    // ternary if
    let result = if i > 0 { "positive" } else { "non-positive" };
    println!("{}", result);

    // while
    while (i != 0) {
        i -= 1;
        println!("Current value is {}", i);
    }

    // loop - iterate with return
    let new_i = loop {
        if (i == 0) {
            break 69;
        }

        i -= 1;
    };
    println!("New value is {}", new_i);

    // for loop
    // looping a slice
    for a in 1..5 {
        println!("{}", a);
    }

    // looping an array
    let names = ["alice", "bob"];
    for name in names.iter() {
        println!("{}", name);
    }

}
