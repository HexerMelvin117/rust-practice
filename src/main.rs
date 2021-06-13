#![allow(dead_code, unused_variables)]
#[path = "teststructs/test_structs.rs"]
mod test_structs;
mod testfuncs;
#[path = "mymods/testingmods.rs"]
mod testingmods;
use testfuncs::say_something;
use text_io::read;

struct Coords {
    latitude: f32,
    longitud: f32,
}

struct Person {
    first_name: String,
    surname: String,
    location: Coords,
}

impl Person {
    fn greet(&self) {
        println!("Hello this is {} {}", self.first_name, self.surname);
    }
}

fn testing_string(word: &str) {
    println!("{}", word)
}

fn main() {
    let test_person = Person {
        first_name: "Melvin".to_string(),
        surname: "Rosales".to_string(),
        location: Coords {
            latitude: 20.5,
            longitud: 30.3,
        },
    };

    test_person.greet();

    let test_string = String::from("Hello yall");
    println!("{}", test_string.to_string());
    println!(
        "My coordinates are {}, {}",
        test_person.location.latitude, test_person.location.longitud
    );

    // Uses greet_user function within mymods/testingmods.rs
    testingmods::greet_user();

    // Prints return value of function add_two_nums within mymods/testingmods.rs
    println!(
        "The result of the sum is {}",
        testingmods::add_two_nums(50, 60)
    );

    let my_book = test_structs::Book {
        author: "Melvin".to_string(),
        title: "Dictionary".to_string(),
    };

    say_something("Hello yall");
    say_something("Hello this seems to work");
    my_book.present_book();
    testing_string("Hello broskis");

    println!("Write Something: ");
    let word: String = read!("{}\n");

    println!("You wrote: {}", word)
}
