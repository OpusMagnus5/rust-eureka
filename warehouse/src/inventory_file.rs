pub mod products; // musimy również dodać pub do deklaracji submodułu
pub use products::ProductCategory as Category; // użycie pub sprawia że eksportuje do wyżej więc można stosować zamiast pub w mod

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory"; // musimy użyć pub aby można było uzyskać dostęp z zewnątrz

#[derive(Debug)]
pub enum ProductCategory {
    Ladder,
    Hammer
}

/*
 domyślnie pola w struct również mają scope private mimo, że struct jest pub, tak samo w impl
 impl nie oznaczamy pub ale już metody tak
*/
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32
}

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how's your coffee?");
}






















