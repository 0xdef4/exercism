use itertools::Itertools;

pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {

        if sides.iter().all(|e| *e > 0) && dbg!(are_valid_sides(sides)) {
            return Some(Self { sides });
        }
        None
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().combinations(2).all(|e| two_elements_are_equivalent(e))
    }

    pub fn is_scalene(&self) -> bool {
        !self.sides.iter().combinations(2).any(|e| two_elements_are_equivalent(e))
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.iter().combinations(2).filter(|e| two_elements_are_equivalent(e.to_vec())).count() >= 1
    }
}

fn are_valid_sides(sides: [u64;3]) -> bool {
    sides.iter().combinations(2).all(|e1| e1.iter().map(|x| **x).sum::<u64>() >= sides.iter().filter(|e2| !e1.contains(e2)).sum::<u64>())
}

fn two_elements_are_equivalent(arr: Vec<&u64>) -> bool {
    if arr.len() != 2 { return false; }
    arr[0] == arr[1]
}
