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

fn main() {
    let rec = Rectangle::new(5, 6);
    let sq = Square::new(7);

    println!("Rect Area is {}", rec.area());
    println!("Rect Area is {}", rec.perimeter());

    println!("Sq Area is {}", sq.area());
    println!("Sq Area is {}", sq.perimeter());
}
