// Generics. Same as Typescript or C#
// T: std::cmp::PartialOrd is more like T extends std::cmp::PartialOrd
// because std::cmp::PartialOrd has comparison ops
// Setting boundaries
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_el = &list[0];

    for el in list {
        if el > largest_el {
            largest_el = el;
        }
    }

    return largest_el;
}

// So if we take the "traits" example
struct Rectangle {
    width: u32,
    height: u32
}

struct Square {
    length: u32
}

impl Square {
    fn new(length: u32) -> Square {
        Square { length }
    }
}

impl Rectangle {
    fn new(height: u32, width: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// Traits are a way to descibe shared functionalities across different structs
// Basically prototype or interface from cpp

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.length * self.length
    }

    fn perimeter(&self) -> u32 {
        self.length * 4
    }
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        self.width * 2 + self.height * 2
    }
}

// Setting trait boundaries through Generics
// Use '+' like AND operator
// fn print_area<T: Shape + Clone>(s: &T) 
fn print_area<T: Shape>(s: &T) {
    println!("Area is {}", s.area())
}

// But this will throw error, if pass 2 rectangles as params
// fn combine_perimeters<T: Shape, U: Shape>(o1: T, o2: U) -> u32 {
// So we could use it like this, which works for any combination
fn combine_perimeters(o1: &(impl Shape), o2: &(impl Shape)) -> u32 {
    o1.perimeter() + o2.perimeter()
}

fn main() {
    println!("Hello, world!");

    // Traits example
    let rec = Rectangle::new(5, 6);
    let sq = Square::new(7);

    print_area(&rec);
    println!("Rect Area is {}", rec.perimeter());

    print_area(&sq);
    println!("Sq Area is {}", sq.perimeter());

    println!("Combined permieter: {}", combine_perimeters(&rec, &sq));
}
