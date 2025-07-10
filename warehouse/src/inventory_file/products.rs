use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer
}

#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32
}

impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self { // musi byc pub jeśli chcemy dostęp z zewnątrz
        super::talk_to_manager(); // sięgamy do rodzica za pomocą super
        Self {
            name,
            category,
            quantity
        }
    }
}