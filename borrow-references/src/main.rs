fn main() {
    let s1 = String::from("hehe");
    let mut s2 = String::from("hehe2");
    
    // Now ownership is transfered and accessing through s1 will throw error
    // println!("{}", calc_len(s1));
    // println!("{s1}");

    // Another hacky way to do it clone.
    // But unecessarily we have a duplicate
    println!("{}", calc_len(s1.clone()));
    println!("{s1}");

    /*
        Borrow reference would basically give temporary ownership to a value

        Rules:
        1. You can make n number of immutable borrows
        2. If you have a mutable borrow in scope, it can be the only borrow in scope
     */
    println!("{}", calc_len_borrow(&s1));
    println!("{s1}");

    // mutable reference
    println!("{}", calc_len_and_add(&mut s2));
    println!("{s2}");

    // to create multiple mutable borrows, create it scope
    {
        let s2_br_1 = &mut s2;
        println!("{}", calc_len_and_add(s2_br_1));
        println!("{s2_br_1}");
    }
    
    let s2_br_2 = &mut s2;
    println!("{}", calc_len_and_add(s2_br_2));
    println!("{s2_br_2}");
    
}

fn calc_len(str: String) -> usize {
    // statement without semicolon is an implied return
    str.len() // return str.len();
}

fn calc_len_borrow(str: &String) -> usize {
    return str.len();
}

// Mutable refrence to a string
fn calc_len_and_add(str: &mut String) -> usize {
    str.push_str(" and me...");
    return str.len();
}
