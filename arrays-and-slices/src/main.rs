fn do_something(slice: &[i32]) {
    // println!("{}", slice); -- for primitive types
    // println!("{:?}", slice); -- for dynamic
    println!("{:?}", slice);
}

fn do_something_mut(slice: &mut [i32]) {
    slice[1] = 2;
    println!("{:?}", slice);
}

fn main() {
    // Array
    // Syntax: let variable: [type; size] = [...values];
    // here [0; 500] sets values of zero to all 500 elements
    let mut x: [i32; 500] = [0; 500];

    x[0] = 1;

    // Edit outside the array bounds, fail compilation
    // At runtimr, the code will panic
    // x[501]= 200
    // To not let it panic, we could use get method, to get result in Option type
    // The Option type will either be Some or None
    let f = x.get(501);

    // Slice
    // What if we want to take a part of the array
    // Not copy it, just reference to a part of it
    // let y = &x[10..20];

    // y[0] = 2; will fail.
    // Hence we need to do a mutable borrow
    let mut y = &mut x[10..20];
    y[0] = 2;

    do_something(y);
    do_something_mut(y);

    println!("Length: {}", x.len());
}
