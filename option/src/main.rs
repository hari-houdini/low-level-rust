fn maybe_divide_by_two(n: u32) -> Option<u32> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None
    }
}


fn main() {
    let result = maybe_divide_by_two(10);

    if let Some(val) = result {
        println!("Result is {}", val);
    } else {
        println!("No value returned");
    }

    match result {
        Some(v) => println!("Got value: {}", v),
        None => println!("No value returned"),
    }

    // otherwise
    let value = match result {
        Some(v) => v,
        None => 0,
    };
    println!("Result we got is {}", value);
    println!("Or using its api: {}", result.unwrap_or(0));
}
