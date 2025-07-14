pub mod inventory_file;
pub mod orders;

pub use inventory_file::{ Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER };
pub use orders::MANAGER as ORDERS_MANAGER;

// ============================================================================================== //

pub mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";
    pub fn ask_about_program() {
        println!("The nutritionist is {NUTRITIONIST}")
    }
}

pub mod weightlifting;
pub mod cardio;

pub use cardio::CardioTool;
pub use cardio::Exercise as CardioExercise;
pub use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise
}

impl GymWorkout {
    pub fn new(cardio: CardioExercise, weightlifting: WeightliftingExercise) -> GymWorkout {
        diet::ask_about_program();
        cardio::ask_about_program();
        weightlifting::ask_about_program();
        
        GymWorkout { cardio, weightlifting }
    }
}