#![allow(dead_code, unused_variables)]

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
    )
}
