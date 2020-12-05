use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

const PASSPORT_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
const BATCH_FILE: &'static str = "../inputs/4.1.txt";

pub fn get_num_valid_passports() -> Result<usize, Box<dyn Error>> {
    let expected_fields = HashSet::from_iter(PASSPORT_FIELDS.iter().copied());
    let batches = fs::read_to_string(BATCH_FILE)?;
    let docs = batches.split("\n\n");
    let mut num_valid = 0;
    for doc in docs {
        let sanitised = doc.replace("\n", " ");
        let entries = sanitised.split(" ");
        let fields: HashSet<&str> = entries
            .map(|e| e.split(":").nth(0).expect("Malformed batch file"))
            .collect();
        let diff: Vec<&str> = expected_fields.difference(&fields).copied().collect();
        if diff.len() == 0 || (diff.len() == 1 && *diff.get(0).unwrap() == "cid") {
            num_valid += 1;
        }
    }
    Ok(num_valid)
}
