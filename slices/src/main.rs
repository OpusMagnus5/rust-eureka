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

fn main() {
    
}















































