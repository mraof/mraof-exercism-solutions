use Allergen::*;

pub struct Allergies(u32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 0,
    Peanuts = 1,
    Shellfish = 2,
    Strawberries = 3,
    Tomatoes = 4,
    Chocolate = 5,
    Pollen = 6,
    Cats = 7,
}

const ORDER: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score & 0xFF)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let i = *allergen as u32;
        ((self.0 >> i) & 1) == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (&ORDER)
            .iter()
            .enumerate()
            .filter_map(|(i, &a)| {
                if ((self.0 >> i) & 1) == 1 {
                    Some(a)
                } else {
                    None
                }
            })
            .collect()
    }
}
