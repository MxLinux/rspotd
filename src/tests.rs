use std::{collections::BTreeMap, error::Error};

use crate::{
    generate, generate_multiple, seed_to_des,
    vals::{DEFAULT_DES, DEFAULT_SEED},
    MaybePotD, PotD,
};

#[test]
fn struct_test() {
    let maybe = MaybePotD::new("2021-12-25", "asdf");
    let potd: Result<PotD, Box<dyn Error>> = maybe.try_into();
    if potd.is_err() {
        println!("{}", potd.unwrap_err());
    } else {
        println!("{:?}", potd.unwrap());
    }
}

#[test]
fn default_seed() {
    assert_eq!(generate("2021-12-25", DEFAULT_SEED).unwrap(), "ZCARK8TPK5");
}

#[test]
fn custom_seed() {
    assert_eq!(generate("2021-12-25", "1122AABB").unwrap(), "FEWNX8OS0O");
}

#[test]
fn new_iter() {
    assert_eq!(generate("2024-04-21", "ASDF").unwrap(), "08GY8HS1RH");
}

#[test]
fn different_date() {
    assert_eq!(generate("1960-10-22", DEFAULT_SEED).unwrap(), "WGAR88TPKS");
}

#[test]
fn small_multiple() {
    let mut comparison_map: BTreeMap<String, String> = BTreeMap::new();
    let date_begin: String = "2021-12-25".to_string();
    let date_end: String = "2021-12-26".to_string();
    let potd_begin: String = "ZCARK8TPK5".to_string();
    let potd_end: String = "ZOU3MLLZO4".to_string();
    comparison_map.insert(date_begin, potd_begin);
    comparison_map.insert(date_end, potd_end);
    assert_eq!(
        generate_multiple("2021-12-25", "2021-12-26", DEFAULT_SEED).unwrap(),
        comparison_map
    );
}

#[test]
fn des_test() {
    assert_eq!("3F.94.E2.AA.46.63.AA.78", seed_to_des("ABCD").unwrap());
}

#[test]
fn small_multiple_invalid_range() {
    assert!(generate_multiple("2021-12-26", "2021-12-25", DEFAULT_SEED).is_err());
}

#[test]
fn small_range_invalid_dates() {
    assert!(generate_multiple("2022-14-12", "2022-12-12", DEFAULT_SEED).is_err());
    assert!(generate_multiple("2022-04-12", "2022-04-32", DEFAULT_SEED).is_err());
}

#[test]
fn multiple_exceed_year() {
    assert!(generate_multiple("2021-12-25", "2022-12-26", DEFAULT_SEED).is_err());
}

#[test]
fn small_seed() {
    assert!(generate("1960-10-22", "ABC").is_err());
}

#[test]
fn large_seed() {
    assert!(generate("1960-10-22", "ABCABCABC").is_err());
}

#[test]
fn invalid_date() {
    assert!(generate("Phoenix dactylifera", DEFAULT_SEED).is_err());
}

#[test]
fn des_seed_short() {
    assert!(seed_to_des("ABC").is_err());
}

#[test]
fn default_seed_des() {
    assert_eq!(seed_to_des(DEFAULT_SEED).unwrap(), DEFAULT_DES);
}

#[test]
fn date_parse_failure() {
    assert!(generate("1999-10-40", DEFAULT_SEED).is_err());
}
