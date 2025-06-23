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
enum PaymentMethodType {
    CreditCard(String), // możemy podać wiele typów wartości, przypomina tuple
    DebitCard(String),
    PayPal(String, String), // ilość wartości nie musi się zgadzać
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
    my_payment_method = PaymentMethodType::PayPal(String::from("bob@gmail.com"), String::from("abc123"));
}

fn main() {
    
}
























