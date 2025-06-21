fn slices() {
    /*
     Slice jest referencją do części (może być również to cała kolekcja) jakiejś kolekcji / tablicy / tuple / stringa
     Jako referencja Slice nie zabiera ownera kolekcji.
    */
}

fn string_slices() {
    let action_hero = String::from("Arnold Schwarzenegger");
    /*
     Podajemy zakres w bajtach, niekoniecznie musi się to pokrywać z literami
    */
    let first_name: &str = &action_hero[0..6]; // exclude 6, first 6 bajtów, możemy to zapisać też [..6]
    println!("{first_name}");
    let last_name: &str = &action_hero[7..21]; // możemy to zapisać też [7..]
    println!("{last_name}");
    let full_name = &action_hero[..]; // slice do całego literału, czyli praktycznie cała referencja
    
    let action_hero = "Arnold Schwarzenegger"; // można powiedzieć że to też jest Slice tylko do całości
    let first_name: &str = &action_hero[..6]; // nadal to będzie &str
    println!("{first_name}");
    let last_name: &str = &action_hero[7..];
    println!("{last_name}");
    
    /*
     Teoretycznie action_hero powinno zostać deallocated ale literał nie jest zapisywany na heapie
     tylko w binarce więc nawet po wyjściu ze scope slice first_name nadal będzie wskazywać prawidłowo
     na Arnolda
    */
    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6];
    };
    
    /*
     Długość string slice odnosi się do liczby bajtów nie liczby znaków.
    */
    let food = "🍕";
    println!("{}", food.len()); // 4 bajty
    /*
     Error - nie można robić slice przerywając ciągłość jednego znaku
    */
    let pizza_slice = &food[0..3]; 
    println!("{}", pizza_slice.len());
    
}

// ============================================================================================== //

fn string_slices_as_a_function_parameters() {
    let action_hero = String::from("Arnold Schwarzenegger");
    /*
     To teoretycznie nie powinno zadziałać bo przekazujemy &String, ale w Rust jesto coś takiego jak
     'deref coercion' - kiedy Rust widzi referencje próbuje ją z dereferencjować dopóki nie dostnie
     oryginalnej wartości. Dzieje się tak że &String może być reprezentowany przez &str ale nie na odwrót.
    */
    do_hero_stuff(&action_hero);
    
    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}

// ============================================================================================== //

fn array_slices() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];
    /*
     W porównaniu do tablicy gdzie jest zadeklarowana jej długość, slice jej nie ma zadeklarowanej
     i może być dynamiczna.
    */
    let my_slice: &[i32] = &values[0..3];
    println!("{my_slice:?}");
}

/*
 To samo co w przypadku String coercion
*/
fn deref_coercion_with_array_slices() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];
    let regular_reference = &values;
    let slice_of_three = &values[..3];
    print_length(regular_reference);
    print_length(slice_of_three);
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}

// ============================================================================================== //

/*
 Rust nie pozwala na mutable string slices, ale pozwala na mutable slice w tablicach.
*/
fn mutable_array_slices() {
    let mut my_array: [i32; 5] = [10, 15, 20, 25, 30];
    let my_slice = &mut my_array[2..4];
    println!("My slice: {:?}", my_slice);
    
    my_slice[0] = 100; // można zmieniać wartości w slice ale w oryginalnej tabeli również się zmienią
    println!("My slice: {:?}", my_slice);
    println!("My array: {:?}", my_array);
}

// ============================================================================================== //

/*
Define a `cereals` array with 5 heap Strings
  - Cookie Crisp
  - Cinnamon Toast Crunch
  - Frosted Flakes
  - Cocoa Puffs
  - Captain Crunch
 
Declare a `first_two` variable that extracts a slice
of the first two cereals. Print the slice.
 
Declare a `mid_three` variable that extracts a slice
of the middle three cereals (Cinnamon Toast Crunch
up to and including Cocoa Puffs). Print the slice.
 
Declare a `last_three` variable that extracts a slice
of the last three cereals. Print the slice.
 
Using the `last_three` slice, target the last element
("Captain Crunch") and replace it with "Lucky Charms".
Print the complete `cereals` array.
 
Declare a `cookie_crisp` variable that references the
"Cookie Crisp" String.
 
Declare a `cookie` variable that extracts a slice of
the text "Cookie" from the String. Print the slice.
 
Declare a `cocoa_puffs` variable. Make it a reference
to the "Cocoa Puffs" String (in other words, a &String).
 
Declare a `puffs` variable that extracts a slice of
the text "Puffs" from the String. Print the slice.
*/

fn slices_test() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch")
    ];
    
    let first_two: &[String] = &cereals[..2];
    println!("First two: {first_two:?}");
    
    let mid_three: &[String] = &cereals[1..4];
    println!("Mid three: {mid_three:?}");
    
    let last_three: &mut [String] = &mut cereals[2..];
    println!("Last three: {last_three:?}");
    
    last_three[2] = String::from("Lucky Charms");
    println!("Cereals: {cereals:?}");
    
    let cookie_crisp: &String = &cereals[0];
    println!("Cookie crisp: {cookie_crisp}");
    
    let cookie: &str = &cookie_crisp[0..6];
    println!("Cookie: {cookie}");
    
    let cocoa_puffs: &String = &cereals[3];
    println!("Cocoa puffs: {cocoa_puffs}");
    
    let puffs: &str = &cocoa_puffs[6..];
    println!("Puffs: {puffs}");
}

fn main() {
    slices_test();
}















































