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

fn main() {
    
}
























