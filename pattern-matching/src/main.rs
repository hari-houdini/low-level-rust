enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String)
}

fn main() {
    let x = Message::Move { x: 0, y: 20 };

    // Account for all possibilties of the statement
    let res: i32 = match x {
        Message::Move { x , y } => x + y,
        // could also be a function
        // Message::Move { x , y } => {
        //     do something
        // },

        // underscore represents all the other cases
        // should be at the end mostly
        _ => 32
    };

    // pattern matching through if let, using arrays
    let array: [i32; 500] = [0; 500];
    // let five_o_one = array.get(501); returns an Option

    if let Some(abs) = array.get(501) {
        println!("{} is the value", abs);
    } else {
        println!("Hold on there. Somethings fishy");
    }

    // pattern matching through while let, using stack
    let mut stack = vec![1, 2, 3];

    // No else statement
    while let Some(top) = stack.pop() {
        println!("Top = {}", top);
    }

    /*
        PATTERN MATCHING test snippet
     */
    // pub enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(u8, u8, u8)
    // }

    // pub fn process_message(msg: Message) -> String {
    //     // TODO: Match and return appropriate string
    //     let str = match msg {
    //         Message::Quit => String::from("Quit"),
    //         Message::Move { x , y } => format!("Moving to ({}, {})", x, y),
    //         Message::Write (msg) => format!("Text: {}", msg),
    //         Message::ChangeColor (r, g, b) => format!("Changing color to ({}, {}, {})", r, g, b)
    //     };

    //     return str;
    // }

}
