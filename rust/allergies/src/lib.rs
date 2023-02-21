use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    fn value(&self) -> u32 {
        match *self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        return Allergies {score: score};
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        return (allergen.value() & self.score) == allergen.value();
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        for allergen in Allergen::iter() {
            if self.is_allergic_to(&allergen) {
                allergies.push(allergen);
            }
        }
        return allergies;
    }
}
