struct User {
    name: &'static str,
    age: u8,
    income: u64
}

enum Direction {
    Up,
    Down,
    Right,
    Left
}

// Unlike other langauges where an enum could be associated with a number,
// here it could be any structure associated 
// Very powerful way to describe types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}


fn main() {
    // Tuple
    let point: (i32, u64) = (0, 1);
    let (x, y) = point;

    println!("{}, {}", point.0, point.1);
    println!("{}, {}", x, y);

    // Struct
    let phil = User {
        name: "Phil",
        age: 20,
        income: 30000
    };
    let jason = User {
        name: "Jason",
        age: 16,
        income: 20000
    };
    let cody = User {
        name: "Cody",
        ..jason
    };

    println!("{} is {} years old, earning £{}!", phil.name, phil.age, phil.income);
    println!("{} is {} years old, earning £{}!", jason.name, jason.age, jason.income);
    println!("{} is {} years old, earning £{}!", cody.name, cody.age, cody.income);

    // Sturct tuple or named tuple - less lightweight than tuple
    struct Color (u32, u32, u32);
    let black = Color(0, 0, 0);

    println!("{}, {}, {}!", black.0, black.1, black.2);

    // Enums
    let dir = Direction::Left;
    let msg = Message::Move { x: 10, y: 20 };
    

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {},{},{}", r, g, b),
    }
}
