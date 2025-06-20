
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



fn main() {
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





































