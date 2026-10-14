use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    last_name: String,
    income: u32,
    age: u32
}

fn main() {
    let mut people: HashMap<String, Person> = HashMap::new();

    let name = String::from("Steve");

    // When something is put into a hashmap, it is sort of owned by
    // the hash map. So since name is a key in hash map,
    // we have to clone the name and use it if needed.
    let name_key = name.clone();

    people.insert(name, Person {
        last_name: String::from("Aoki"),
        income: 10000,
        age: 32
    });

    // Not recommended, although could be done
    // println!("{:?}", &people[&name_key]);
    // The reason being, if it does not exist, the compiler will panic
    // println!("{:?}", &people[&String::from("does not exist")]);

    if let Some(x) = people.get(&name_key) {
        println!("{:?}", x);
    } else {
        println!("Does not exist");
    }

    // or use Hash map APIs
    people.entry(name_key)
        .or_insert(Person {
            last_name: String::from("Unknown"), 
            income: 0, 
            age: 0}
        );
}
