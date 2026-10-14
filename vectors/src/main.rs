// Vector is a dynamic list, stored in heap, meta-ed in stack

#[derive(Debug)]
struct Thing {
    x: u32,
    y: u32
}

fn main() {
    let mut v: Vec<Thing> = vec![Thing{ x: 0, y: 0 }];

    // Not recommended, although could be done
    // println!("{:?}\n", v[2]);
    println!("{:?}\n", v.get(2).unwrap_or(&Thing{ x: 1, y: 1 }));

    // If we dont borrow the vector, the vector goes out of scope after the loop
    for i in &v {
        println!("{:?}\n", i);
    }

    for i in &mut v {
        i.x += 10;
        i.y += 20;
        println!("{:?}\n", i);
    }

    for t in v.iter() {
        println!("x = {}, y = {}", t.x, t.y);
    }

    // Before vec goes out of scope and gets dropped,
    // it first drops the values within it and finally itself
    // to avoid mem leak
}
