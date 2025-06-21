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

fn main() {
    
}
