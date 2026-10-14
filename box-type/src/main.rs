// Rust is really bad with linked lists as the
// length of the list is unknown at compile time
// A workaround would be a sort of recursive data structure
enum List {
    // You cant do this: Cons(i32, List)
    Cons(i32, Box<List>),

    // Sort of null pointer
    Nil
}


// pritive types live in stack and not heap.
// so structs that are derived from primitive types also live in stack
// Box<T> would make it a heap based structure.
// The reason being, if it exists in stack, gimme_a_thing() factory should 
// always create a new Thing and pass by value to the place where its called
// and copy it around, which is not efficient
struct Thing {
    x: u32,
    y: u32
}

// Pass by value
fn gimme_a_thing() -> Thing {
    Thing::new()
}

// Pass by reference
fn better_gimme_a_thing() -> Box<Thing> {
    Box::new(Thing::new())
}

impl Thing {
    fn new() -> Self {
        Thing {
            x: 0,
            y: 0
        }
    }
}

fn main() {
    // Returns the copied value of the Thing itself
    let t1 = gimme_a_thing();

    // Returns reference to the heap address where the Box exists
    let mut t2 = better_gimme_a_thing();

    // Deference the Box to get the value of the Thing (*t)
    // And mutate it to a new value t1
    *t2 = t1;
}
