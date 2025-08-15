use std::collections::HashMap;

/*
 Train jest interfejsem jak w Java. Definiuje metody jakie trzeba zaimplementować aby spełnić kontrakt.
*/

trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

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

fn main() {
    
}