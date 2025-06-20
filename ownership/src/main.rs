
/*
 ownership - to feature kompilatora, nie ma impaktu w runtime działającego programu
 To zestaw reguł kompilatora, który sprawdza, czy program jest wolny od błędów pamięci

 Każda wartość w Rust ma ownera, owner wartości może się zmieniać w trakcie działania programu, ale
 zawsze jest tylko jeden w danym momencie.

 Ownerem może być zmienna albo parametr funkcji. Zmienna może być ownerem tuple, ale tuple jest
 ownerem wartości w swojej kolekcji.
*/

/*
 Stack jest szybszą częścią pamięci w programie, ale wspiera tylko dane, których rozmiar można określić
 w czasie kompilacji. Stack działa jak kolejka LIFO. Typy prymitywne takie jak int, float bool, chars,
 arrays mają określony rozmiar, więc Rust może je przechować na stacku.
 Allocator pamięci zwraca referencje do danych. Ta referencja może być przechowywana w zmiennej.
 Referencje mają znany rozmiar, więc można je przechowywać na stacku.

 Heap jest wolniejszy i większy, ale wspiera dane o dynamicznym rozmiarze, który może się zmieniać w trakcie
 działania programu.

*/



fn test() {
    // age jest ownerem value 33 i jest odpowiedzialny za zwolnienie pamięci po wyjściu z bloku
    // najpierw jest czyszczony is_handsome, a potem age zgodnie z LIFO
    let age: i32 = 33; 
    let is_handsome: bool = true;
}

fn copy_trait() {
    /*
     Typy które obsługują copy trait mogą zostać w pełni zduplikowane. Wszystkie typy o znanej
     wielkości w czasie kompilacji implementują copy trait.
    */

    /*
     Do year zostanie przypisana niezależna kopia time. Więc mamy dwóch ownerów które odpowiadają za
     swoje wartości
    */
    let time = 2025;
    let year = time;
}

fn string_type() {
    /*
     Typ &str nie jest przechowywany ani na stacku ani na heapie, jest zapisywany w wynikowym pliku
     executable, ponieważ jest używany tylko gdy wartość jest znana już w trakcie kompilacji.
    */
    let food: &str = "pasta";

    /*
     Typ String to inny typ od &str. String jest używany gdy jest to dynamiczna wartość np. jako
     input od użytkownika albo z pliku. Ten typ wspiera również operacje które mutują Stringa
     w przeciwieństwie do &str. Ten typ jest przechowywany na heapie.
    */

    // Tworzenie pustego Stringa. Tak się odwołujemy do metod w jakimś namespace.
    let text: String = String::new();
    // Tworzenie Stringa z literału ("KitKat"), literał jest przechowywany w binarce
    let candy: String = String::from("KitKat");
    // text i candy są ownerami i są odpowiedzialne za zwolnienie pamięci

    /*
     Taki string zapisywany jest zarówno na heapie jak i na stacku.
     Na stacku zapisujemy referencje do heapu gdzie jest przechywana wartość, wielkość (length) Stringa w
     bajtach oraz pojemność miejsca w którym został zapisany (capacity) w bajtach.
     Na heapie jest zapisywana tylko wartość.
    */
    let mut name = String::from("Damian");

    /*
     Jeśli capacity było wystarczające String zostaje w tym samym miejscu na heapie.
    */
    name.push_str(" Bodzioch");
}

fn move_ownership() {
    /*
     move - służy do zmiany ownera na innego

     W przypadku poniżej String nie wspiera copy trait i dlatego Rust kopiuje jedynie dane z stacku
     czyli referencji, długość i pojemność. Zmienia się także OWNER z person na genius i to on jest
     odpowiedzialny teraz za zwolnienie pamięci. A także sprawia to że zmienna person staje się
     INVALID i nie możemy jej użyć po deklaracji genius.
    */
    let person: String = String::from("Damian");
    println!("{person}"); // OK
    let genius: String = person;
    // println!("{person}") ERROR
}

fn drop_function() {
    let person = String::from("Damian");
    /*
    na koniec scope przy zwalnianiu zasobów rust wywołuje funkcję drop()
    drop nie działa na pamięć na stacku tylko na heapie
    */

    drop(person); // możemy wywołać ją ręcznie i zmienna staje się INVALID
}

fn clone_method() {
    /*
     Rust przez ownersip stara się unikać duplikacji heap danych ze względu na szybkość i
     optymalizacje pamięci.
     Ale istnieje trait Clone który pozwala na duplikowanie typów które domyślnie nie są
     duplikowane przez Rust o ile implementują Clone trait.
    */

    let person = String::from("Damian");
    let genius = person.clone(); // Duplikacja danych
    println!("{person}") // OK
}

fn references_and_borrowing() {
    /*
     Referencje pozwalają używać wartości bez konieczności przenoszenia ownera.
     Właściwość tę nazywamy - borrowing.
     Deklarujemy że pożyczamy wartość tworząc do niej referencje ale zwrócimy ją gdy skończymy
     jej używać.
    */

    let my_stack_value = 2;
    let my_integer_reference: &i32 = &my_stack_value; // & - za pomocą tego operatora pożyczamy

    let my_heap_value = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;

    /*
     Na koniec działania scope, value jest ownerem wartości która zostanie zwolniona, a
     reference jest ownerem tylko referencji i która zostanie wyczyszczona.

     Referencja jest jednym z typów wskaźników (Pointers), wskazuje na valid value jeśli chodzi o
     istnienie i życie tej wartości.
     Sam wskaźnik nie posiada takiej gwarancji.
    */
}

fn dereference() {
    /*
     dereference - służy do uzyskania wartości na którą wskazuje referencja.
    */

    let my_heap_value = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;
    /*
     W obu przypadkach wyświetlimy Toyota ponieważ referencja implementuje Display trait który
     robi dereference aby dostać się do wartości.
    */
    println!("{}", *my_heap_reference); // deference robimy za pomocą *
    println!("{}", my_heap_reference);

    // &str to referencja do literału w binarce a samo str to tylko read-only text z binarki
    let ice_cream: &str = "Cookies and Cream";
}

fn copy_references() {
    /*
     Rust robi kopię referencji więc nie zmiany ownera, ponieważ są to referencje są zapisane na stacku.
    */
    let ice_cream: &str = "Cookies and Cream";
    let dessert = ice_cream;
    println!("{ice_cream}"); // OK
}

fn ownership_and_function_parameters() {
    let apples = 6;
    /*
     Tutaj nie ma zmiany ownera. Robiona jest kopia wartości ze stacka tak jak w przypadku:
     let value = apples;
    */
    print_my_value(apples);

    /*
     Tutaj jest zmiana ownera. Parametr funkcji przejmuje ownera oranges.
    */
    let oranges: String = String::from("Oranges");
    print_my_string(oranges);
    // println!("{oranges}"); oranges is INVALID
}

fn print_my_value(value: i32) {
    println!("Your value is {value}");
}

fn print_my_string(value: String) {
    println!("You value is {value}");
}

fn mutable_parameters() {
    let burger = String::from("Burger");
    add_fries(burger);
}

fn add_fries(mut meal: String) {
    // meal.push_str(" and Fries"); not work with immutable parameter
    meal.push_str(" and Fries");
}

fn return_values() {
    /*
     Przekazanie ownera cake z funkcji bake_cake do cake w tej funkcji.
    */
    let cake = bake_cake();
    println!("{cake}");

    let current_meal = String::from("Meal");
    add_flour(current_meal);
    // add_sugar(current_meal); Error - owner został zmieniony po przekazaniu do funkcji

    // to jedne z rozwiązań takiej sytuacji ale później będzie info jak rozwiązać to referencjami
    let mut current_meal_2 = String::from("Meal");
    current_meal_2 = add_flour_2(current_meal_2);
    current_meal_2 = add_sugar_2(current_meal_2);
}

fn bake_cake() -> String {
    let cake = String::from("Chocolate Mousse");
    cake
}

fn add_flour(mut meal: String) {
    meal.push_str("Add flour");
}

fn add_sugar(mut meal: String) {
    meal.push_str("Add sugar");
}

fn add_flour_2(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

fn add_sugar_2(mut meal: String) -> String {
    meal.push_str("Add sugar");
    meal
}

/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.
 
The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.
 
In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.
 
Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

fn main() {
    // brak przeniesienia ownera, kopia wartości
    let is_concert = true;
    let is_event = is_concert;
    println!("{is_concert}");
    println!("{is_event}");

    // brak przeniesienia ownera, kopia wartości
    let sushi = "Salmon";
    let dinner = sushi;
    println!("{sushi}");
    println!("{dinner}");

    let sushi = String::from("Salmon");
    let dinner = sushi;
    // println!("{sushi}"); ERROR - owner przeniesiony do dinner
    println!("{dinner}");
    
    /*
     dinner zmienia ownera na meal w funkcji eat_meal, po zakończeniu funkcji meal jest utilizowany
     a dinner jest INVALID
    */
    eat_meal(dinner);
    // println!({dinner}); ERROR - owner przeniesiony do argumentu funkcji 
    
    /*
     Gdybyśmy chcieli zatrzymać wartość po eat_meal, funkcja musiałaby zwrócić wartość czyli
     przekazać ownera do dinner a dinner musiałaby być mutable.
    */
}

fn eat_meal(mut meal: String) {
    meal.clear()
}

































