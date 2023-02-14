use enum_iterator::{all, Sequence};

pub struct Allergies{
    score : u32,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Sequence)]
pub enum Allergen {
    Eggs            = 1,
    Peanuts         = 2,
    Shellfish       = 4,
    Strawberries    = 8,
    Tomatoes        = 16,
    Chocolate       = 32,
    Pollen          = 64,
    Cats            = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = *allergen as u32;
        if allergen_score == 0 {
            return false;
        }

        let mask = self.score & allergen_score;
        if mask != 0 {
            return true;
        }
        return false;
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let vec = all::<Allergen>().collect::<Vec<Allergen>>();

        vec.into_iter().filter(|x| self.is_allergic_to(x)).collect()
    }
}
