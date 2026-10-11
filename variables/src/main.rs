use std::sync::Mutex;
// cargo add lazy_static
use lazy_static::lazy_static;

// This is a string slice
static HELLO: &str = "Hello world";

// There are no automatic protections against data races or undefined behavior in static muts
// If you need global mutable state safely, use interior mutability 
// patterns like static with Mutex<T> or AtomicUsize:
lazy_static! {
    static ref COUNTER: Mutex<u32> = Mutex::new(0);
}

fn main() {
    println!("{}", HELLO);

    // let name: type = default_value;
    // let x = 2;
    // x = 3; // error thrown as its immutable

    let mut x = -1;

    let y: u64 = 2; // unsigned 64bit int
    const PI: f32 = 3.14; // const float 32 bit

    // Difference between a `const` and an immutable `let` is that
    // const value has to be known at compile time
    // const z = y * 2; would throw an error

    // Static variable should be uppercase
    // Has a lifetime of static (or) program itself
    static SX: i32 = 3;

    // But if I want to update a static variable,
    // which is unsafe and could cause many issues like race conditions
    // I could only use it within unsafe block.
    static mut UNSAFE_SX: i32 = 34;
    // println!("{}", UNSAFE_SX); throws an error due to use of mutable static
    unsafe {
        // println!("{}", UNSAFE_SX); Even then it cannot be access as is
        println!("{}", *&raw const UNSAFE_SX);
    }

    let mut count = COUNTER.lock().unwrap();
    *count += 1;

    x += SX;

    // Shadowing - redeclaring a variable with the same name, without needing mut.
    // This creates a new binding that overrides the previous one.
    let x = x + 1;

    println!("{} {} {}", x, y ,PI);
}
