use std::collections::HashMap;

/*
 Train jest interfejsem jak w Java. Definiuje metody jakie trzeba zaimplementować aby spełnić kontrakt.
*/

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

/* ============================================================================================== */

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new()
        }
    }
    
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

// implementacja dla Hotel
impl Accommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

/* ============================================================================================== */

#[derive(Debug)]
struct AirBnB {
    host: String,
    guest: Vec<(String, u32)>
}

impl AirBnB {
    fn new(host: &str) -> AirBnB {
        Self {
            host: host.to_string(),
            guest: vec![]
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guest.push((name.to_string(), nights));
    }
}

fn implementing_trait() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);
    println!("{hotel:#?}");
    
    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{airbnb:#?}")
}

/* ============================================================================================== */

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

fn traits_for_function_parameter_constraints() {
    let mut hotel = Hotel::new("The Luxe");
    hotel.book("Piers", 5);
    book_for_one_night(&mut hotel, "Damian");
    println!("{hotel:#?}");
}

/* ============================================================================================== */

fn book_for_one_night_2<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// tu możemy przekazać dwie różne implementacje Accommodation
fn mix_and_match(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// tutaj muszą być to dwie takie same implementacje Accommodation
fn mix_and_match_2<T: Accommodation>(first: &mut T, second: &mut T, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// a tu wspieramy dwa rożne typy Accommodation
fn mix_and_match_3<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

/* ============================================================================================== */

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay!")
    }
}

impl Description for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of the luxury", self.name)
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

fn mix_and_match_4(
    first: &mut (impl Accommodation + Description), // musi również wspierać Description
    second: &mut impl Accommodation, 
    guest: &str
) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// przykład z generics
fn book_for_one_night_3<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

/* ============================================================================================== */

/*
 za where możemy zdefiniować generyki zamiast w < >
*/
fn mix_and_match_5<T, U>(
    first: &mut T, // musi również wspierać Description
    second: &mut U,
    guest: &str
) where
    T: Accommodation + Description,
    U: Accommodation
{
    first.book(guest, 1);
    second.book(guest, 2);
}

fn main() {
    implementing_trait();
    traits_for_function_parameter_constraints();
}



































