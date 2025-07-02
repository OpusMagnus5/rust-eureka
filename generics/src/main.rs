/*
 <T> - definicja typu generycznego
 T - może być to dowolna litera bądź nawet nazwa
 Pod spodem tak naprawdę nie ma żadnego mechanizmu generyczności. W wynikowym pliku dla każdego
 typu użytego w funkcji zostaje rozpisana funkcja przy użyciu danego typu.
 
 Turbofish operator pozwala nadpisać defaultowy typ
*/
fn identity<T>(value: T) -> T {
    value
}

fn intro_to_generics() {
    println!("{}", identity::<i64>(5)); // nadpisujemy i32 przez i64
    println!("{}", identity(5.77));
}

// ============================================================================================== //

fn multiple_generics() {
    make_tuple("hello", 5);
}

/*
 <T, U> deklaracja dwóch różnych typów
*/
fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// ============================================================================================== //

fn generics_in_structs() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold"
    };
}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }    
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> { // Generyczna implementacja
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn generics_iand_impl() {
    let mut silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: String::from(" Silver ")
    };
    silver_chest.clean_treasure();

    let special_chest: TreasureChest<[&str; 3]> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: ["GOld", "Silver", "Platinium"]
    };
    special_chest.amount_of_treasure();
    
    silver_chest.capital_captain();
    special_chest.capital_captain();
}

// ============================================================================================== //

enum Cheesesteak<T> {
    Plain, // nie każdy enum musi wykorzystywać T
    Topic(T)
}

fn generics_and_enums() {
    let mushroom: Cheesesteak<&str> = Cheesesteak::Topic("mushroom");
    // mimo że nie wykorzystuje T musimy zadeklarować konkretny typ generyczny
    let cheesesteak: Cheesesteak<String> = Cheesesteak::Plain; 
}

// ============================================================================================== //

/*
Let's model a real-time chat system where users can
share audio and video files.
 
Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation.
 
Define a ChatMessage struct with two fields: `content`
and `time`. The struct should define one generic type, T,
which will be the type of the `content` field.
The `time` field should always be a String.
Derive a Debug implementation.
 
Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile".
 
Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct.
 
In `main`, create a ChatMessage with `content` set to a
string slice.
 
Create another ChatMessage with `content` set to a String.
 
Create another ChatMessage with `content' set to a
DigitalContent variant.
 
Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum.
 
Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn generics_test() {
    let slice_message: ChatMessage<&str> = ChatMessage {
        content: "Slice Hello World",
        time: String::from("12:31")
    };
    slice_message.retrieve_time();
    println!("Message time {}", slice_message.retrieve_time());
    
    let string_message: ChatMessage<String> = ChatMessage {
        content: String::from("Hello World!"),
        time: String::from("12:34")
    };
    println!("Message time {}", string_message.retrieve_time());
    
    let digital_message: ChatMessage<DigitalContent> = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("12:45")
    };
    println!("Message time {}", digital_message.retrieve_time());
    digital_message.consume_entertainment();
}

fn main() {

}




























