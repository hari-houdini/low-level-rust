struct Rectangle {
    width: u32,
    height: u32
}

fn get_area(r: &Rectangle) -> u32 {
    r.width * r.height
}

// Implementations give structs, a method functionality
// Since rust does not have class per se,
// these let structs be classes, in other words

impl Rectangle {
    fn new(height: u32, width: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn new_self(height: u32, width: u32) -> Self {
        Self { width, height }
    }

    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec = Rectangle {
        width: 2,
        height: 3
    };
    let rec_impl = Rectangle::new(5, 6);
    let rec_impl_self = Rectangle::new_self(3, 5);

    println!("Area is {}", get_area(&rec));
    println!("Area is {}", rec.get_area());

    println!("Area is {}", get_area(&rec_impl));
    println!("Area is {}", rec_impl.get_area());

    println!("Area is {}", get_area(&rec_impl_self));
    println!("Area is {}", rec_impl_self.get_area());
}
