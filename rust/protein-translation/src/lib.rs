use std::collections::{HashMap, HashSet};

pub fn get_mappings() -> HashMap<&'static str, HashSet<&'static str>> {
    HashMap::from([
        ("Methionine", HashSet::from(["AUG"])),
        ("Phenylalanine", HashSet::from(["UUU", "UUC"])),
        ("Leucine", HashSet::from(["UUG", "UUA"])),
        ("Serine", HashSet::from(["UCU", "UCC", "UCA", "UCG"])),
        ("Tyrosine", HashSet::from(["UAU", "UAC"])),
        ("Cysteine", HashSet::from(["UGU", "UGC"])),
        ("Tryptophan", HashSet::from(["UGG"])),
        ("STOP", HashSet::from(["UAA", "UAG", "UGA"])),
    ])
}

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut result = Vec::new();
    let mapping = get_mappings();

    for chunk in rna.as_bytes().chunks(3) {
        if chunk.len() < 3 {
            continue;
        }
        let codon = std::str::from_utf8(chunk).unwrap();

        let mut found = false;

        for (protein, codons) in &mapping {
            if codons.contains(codon) {
                if *protein == "STOP" {
                    return Some(result);
                }
                found = true;
                result.push(*protein);
                break;
            }
        }
        if !found {
            return None;
        }
    }

    if rna.len() % 3 != 0 {
        return None;
    }

    Some(result)
}
