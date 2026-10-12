fn funky_town(str: String) {
    println!("{str}");
    // When a variable goes out of scope, Rust automatically calls drop() to free its resources.
}

fn main() {
    /*
        Memory management

        1. Manual - Like malloc/free in cpp
        2. Garbage collector - Like mark and sweep
        3. Ownership - Like Rust
     */

    /*
        Ownership

        1. Every value in memory is owned by something
        2. There can only be one owner at a time
        3. When owner goes out of scope, the value is dropped

        Simple values like integers implement the Copy trait.
        Rust performs a bitwise copy for Copy types. No ownership transfer occurs.
     */

    /*
        Example 1
     */

    // Label s1, address pointer to the heap and other meta data are store in "Stack"
    // Value hehe itself is stored in the "Heap"
    let s1 = String::from("hehe");

    // Now ownership to "hehe" in Heap is given to s2 from s1
    // as there can only be one owner at a time
    let s2 = s1;

    // will throw an error as s2 will hold the meta info to the heap that
    // stores the string value "hehe"
    // println!("{s1}");

    /*
        Example 2
     */

    // Once we pass s2 to funky_town, str becomes the owner
    funky_town(s2);

    // Post execution of the function, the value is dropped.
    // Performing operations on s2 will no longer work.
    // will throw a "value borrowed here after move"
    // println!("{s2}");

    // So a cheeky solution would be return the string and capture it in a new variable.
    // Since it is clunky, thats where "borrow checker"/"pass by reference" comes into picture.
}
