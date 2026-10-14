use std::fs;

fn read_contents(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    // ? operator
    // when the function returns successful, it will reveal the inner value
    // when it fails, it will fall through and implicitly return
    //
    // Example,
    // let contents = fs::read_to_string(path);
    // if let Ok(r) = contents {
    //     return Ok(r);
    // }
    // if let Err(e) = contents {
    //     return Err(e);
    // }

    Ok(contents)
}

fn main() {
    println!("Hello, world!");

    let res = read_contents("./somthing.txt");

    if let Ok(r) = res {
        println!("{}", r.to_string());
    } else {
        println!("Something went wrong");
    }

    // Or the more clean way
    let res2 = read_contents("./somthing.txt");
    match res2 {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }
}
