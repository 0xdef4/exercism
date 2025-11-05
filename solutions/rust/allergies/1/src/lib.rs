pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq)]
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
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut output = Vec::new();
        let mut target = self.0;
        let mut list = (0..).map(|n| 2_u32.pow(n)).take_while(|n| *n <= self.0).collect::<Vec<_>>();
        list.reverse();

        for i in list {
            if target >= i {
                target = target - i;
                output.push(i);
            } else {
                continue;
            }
            if target == 0 {
                break;
            }
        }

        output.sort();
        output.iter().filter_map(|e| match (*e).ilog2() {
            0 => Some(Allergen::Eggs),
            1 => Some(Allergen::Peanuts),
            2 => Some(Allergen::Shellfish),
            3 => Some(Allergen::Strawberries),
            4 => Some(Allergen::Tomatoes),
            5 => Some(Allergen::Chocolate),
            6 => Some(Allergen::Pollen),
            7 => Some(Allergen::Cats),
            _ => None
        }).collect()
    }
}
