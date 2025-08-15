use std::collections::HashMap;
use std::fmt::Display;

/*
 Train jest interfejsem jak w Java. Definiuje metody jakie trzeba zaimplementować aby spełnić kontrakt.
*/

pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

/* ============================================================================================== */

#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new()
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

// implementacja dla Hotel
impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) { // tu nie trzeba pub bo interfejs jest już pub
        self.reservations.insert(name.to_string(), nights);
    }
}

/* ============================================================================================== */

#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guest: Vec<(String, u32)>
}

impl AirBnB {
    pub fn new(host: &str) -> AirBnB {
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

/* ============================================================================================== */

pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay!")
    }
}

impl<T:  Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of the luxury", self.name)
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}