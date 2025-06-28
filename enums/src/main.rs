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
    Whole,
    NonDairy { kind: String }
}

impl Milk {
    fn drink(self) {
        match self {
            /*
             matchuje tylko dany enum z daną wartością
            */
            Milk::LowFat(2) => println!("Delicious"),
            Milk::Whole => println!("Whole milk"),
            Milk::LowFat(percent) => println!("Low Fat {percent}"),
            _ => println!("Different")
        }
    }
}

// ============================================================================================== //

/*
 if let to kombinacja ifa z deklaracją zmiennej
*/

fn if_let_combination() {
    let my_beverage = Milk::Whole;
    
    if let Milk::Whole = my_beverage { // jeśli zmienna my_beverage jest równa Milk Whole
        println!("You have whole milk");
    }
    
    if let Milk::LowFat(percent) = my_beverage {
        println!("You have whole milk with {percent}"); // mogę użyć powiązanej właściwości
    }
    
    if let Milk::NonDairy { kind } = my_beverage {
        println!("You have whole milk with {kind}");
    }
}

fn let_else_combination() {
    let my_beverage = Milk::Whole;
    
    let Milk::LowFat(percent) = my_beverage else {
        println!("Not LowFat"); // wykona się tylko gdy nie jest czyli zaprzeczenie
        return;
    };

    print!("{percent}"); // Zmienna jest dostępna tylko poza blokiem
}

// ============================================================================================== //

/*
Define a Tier enum with three variants: Gold, Silver,
Platinum. Derive a Debug implementation for the Tier enum.
 
Define a Subscription enum with three variants: Free,
Basic, and Premium. Derive a Debug implementation for the
Subscription enum.
 
The Free variant should have no associated data.
 
The Basic variant should be a tuple variant with two pieces
of data. The first one should be a f64 (the price per month)
and the second one should be a u32 (the number of months).
 
The Premium variant should be a struct variant with a 'tier'
field. The tier field should be a Tier enum value.
 
Define a 'summarize' method on the Subscription enum.
 
If the Subscription is Free, output the text "You have
limited access to the site".
 
If the Subscription is Basic, output the text "You have
limited access to the site's premium features for {price}
for {months} months", where {price} amd {months} come from
the tuple variant's associated data.
 
If the Subscription is Premium, output the text "You have
full access to the site's premium features. Your tier is
{tier:?}"", where {tier} comes from the struct variant's
associated 'tier' field.
 
In the `main` function, create 3 instances of Subscription,
each one with a different variant. Invoke the `summarize`
method on each instance.
*/

#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier }
}

impl Subscription {
    fn summarize(&self) {
        match self { 
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price, months) => {
                println!("You have limited access to the site's premium features for {price} for {months} months");
            },
            Subscription::Premium { tier } => {
                println!("You have full access to the site's premium features. Your tier is {tier:?}");
            }
        }
    }
}

fn test() {
    let free_sub = Subscription::Free;
    let basic_sub = Subscription::Basic(12.63, 1);
    let premium_sub = Subscription::Premium { tier: Tier::Platinum };
    
    free_sub.summarize();
    basic_sub.summarize();
    premium_sub.summarize();
}

fn main() {
    
}
























