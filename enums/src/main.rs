#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

fn intro_to_enums() {
    let first_card = CardSuit::Hearts; // inicjalizacja enuma
}

// ============================================================================================== //
#[derive(Debug)]
struct Credentials {
    username: String,
    password: String
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String), // możemy podać wiele typów wartości, przypomina tuple
    DebitCard(String),
    PayPal(String, String), // ilość wartości nie musi się zgadzać
    PayPal2(Credentials), // można użyć structa aby ustrukturyzować dane
    PayPalStruct { username: String, password: String } // wersja struct
}

/*
 Rust wybierze rozmiar pamięci dla wszystkich Enumów na podstawie tego z największą ilością zajmowanego przez niego
 i jego zmienne miejsca.
*/

fn enums_with_associated_values() {
    /*
     Nie przypomina to Javy gdzie wartość przypisana do pola enuma jest stała.
    */
    let mut my_payment_method: PaymentMethodType = PaymentMethodType::CreditCard(String::from("0100-4528"));
    
    let paypal_credentials = Credentials {
        username: String::from("bob@gmail.com"),
        password: String::from("abc123"),
    };
    my_payment_method = PaymentMethodType::PayPal2(paypal_credentials);
    
    my_payment_method = PaymentMethodType::PayPalStruct{ 
        username: String::from("bob@gmail.com"),
        password: String::from("abc123"), 
    };
}

// ============================================================================================== //

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito(Meat),
    Bowl(Meat),
    VeganPlate { meat: Meat }
}

fn nesting_enums_with_enums() {
    let mut lunch = RestaurantItem::Burrito(Meat::Chicken);
    lunch = RestaurantItem::VeganPlate { meat: Meat::Steak };
}

// ============================================================================================== //

enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String)
}

impl LaundryCycle {
    fn wash_laundry(&self) { // zasady dla self są takie same jak dla struct
        match self {
            LaundryCycle::Cold => {
                println!("Running teh laundry with cold temp.");
            },
            LaundryCycle::Hot { temperature} => { // referencja do temperatury czyli borrowing ownership
                println!("Running teh laundry with {temperature} temp.");
            },
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running teh laundry for {fabric_type}");
            }
        }
    }
}

fn match_and_enums() {
    let my_computer = OperatingSystem::Windows;
    let age: u32 = years_since_release(my_computer);
    println!("My system is {age} old");
    
    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot { temperature: 60 });
    wash_laundry(LaundryCycle::Delicate(String::from("Wool")));
    
    LaundryCycle::Cold.wash_laundry();
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os { 
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34
    }
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle { 
        LaundryCycle::Cold => {
            println!("Running teh laundry with cold temp.");
        },
        LaundryCycle::Hot { temperature} => { // w ten sposób możemy uzyskać dostęp do danych Enuma
            println!("Running teh laundry with {temperature} temp.");
        },
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running teh laundry for {fabric_type}");
        }
    }
}

// ============================================================================================== //

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self { 
            OnlineOrderStatus::Delivered | OnlineOrderStatus::Packed => println!("Your item has been delivered"), // kilka enumów w jednym
            other_status => println!("Your item is not there yet {other_status:?}") // zamiast _ możemy użyć zmiennej
        }
    }
}

// ============================================================================================== //

enum Milk {
    LowFat(i32),
    Whole
}

impl Milk {
    fn drink(self) {
        match self {
            /*
             matchuje tylko dany enum z daną wartością
            */
            Milk::LowFat(2) => println!("Delicious"),
            Milk::Whole => println!("Whole milk"),
            Milk::LowFat(percent) => println!("Low Fat {percent}")
        }
    }
}

fn main() {
    
}
























