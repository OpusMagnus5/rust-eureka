use std::collections::HashMap;

/*
 Train jest interfejsem jak w Java. Definiuje metody jakie trzeba zaimplementować aby spełnić kontrakt.
*/

trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay!")
    }
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
}

// implementacja dla Hotel
impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of the luxury", self.name)
    }

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
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guest.push((name.to_string(), nights));
    }
}

fn implementing_trait() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.get_description());
    hotel.book("Piers", 5);
    println!("{hotel:#?}");
    
    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{airbnb:#?}")
    
    
}

fn main() {
    implementing_trait();
}



































