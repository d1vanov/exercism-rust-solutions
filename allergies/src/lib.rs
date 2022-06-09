pub struct Allergies {
    score: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_value = u32::pow(2, *allergen as u32);
        self.score & allergen_value != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        const ALL_ALLERGENS: [Allergen; 8] = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        let mut result = Vec::new();
        for allergen in ALL_ALLERGENS {
            if self.score & u32::pow(2, allergen as u32) != 0 {
                result.push(allergen);
            }
        }

        result
    }
}
