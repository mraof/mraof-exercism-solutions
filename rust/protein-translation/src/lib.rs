use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        if rna.len() % 3 != 0 {
            None
        } else {
            let mut vec = Vec::new();
            let mut iter = rna.chars();
            while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
                let codon: String = [a, b, c].iter().collect();
                if let Some(name) = self.name_for(&codon) {
                    if name == "stop codon" {
                        break;
                    }
                    vec.push(name)
                } else {
                    return None;
                }
            }
            Some(vec)
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        map: pairs.into_iter().collect()
    }
}
