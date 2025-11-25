#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(index) = dna.find(|c| c != 'A' && c != 'G' && c != 'C' && c != 'T') {
            return Err(index);
        }
        Ok(Self(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let mut rna_string = String::new();
        for c in self.0.chars() {
            match c {
                'G' => rna_string.push('C'),
                'C' => rna_string.push('G'),
                'T' => rna_string.push('A'),
                'A' => rna_string.push('U'),
                _ => unreachable!()
            }
        }
        Rna::new(rna_string.as_str()).expect("RNA transformation should work")
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(index) = rna.find(|c| c != 'A' && c != 'G' && c != 'C' && c != 'U') {
            return Err(index);
        }
        Ok(Self(rna.to_string()))
    }
}
