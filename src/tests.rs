use crate::{generate, generate_multiple, seed_to_des, vals::DEFAULT_SEED};

#[test]
fn default_seed() {
    assert_eq!(generate("2021-12-25", DEFAULT_SEED), "ZCARK8TPK5");
}

#[test]
fn custom_seed() {
    assert_eq!(generate("2021-12-25", "1122AABB"), "FEWNX8OS0O");
}

#[test]
fn different_date() {
    assert_eq!(generate("1960-10-22", DEFAULT_SEED), "WGAR88TPKS");
}

#[test]
fn small_multiple() {
    use std::collections::HashMap;
    let mut comparison_map: HashMap<String, String> = HashMap::new();
    let date_begin: String = "2021-12-25".to_string();
    let date_end: String = "2021-12-26".to_string();
    let seed_begin: String = "ZCARK8TPK5".to_string();
    let seed_end: String = "ZOU3MLLZO4".to_string();
    comparison_map.insert(date_begin, seed_begin);
    comparison_map.insert(date_end, seed_end);
    assert_eq!(
        generate_multiple("2021-12-25", "2021-12-26", DEFAULT_SEED),
        comparison_map
    );
}

#[test]
fn des_test() {
    assert_eq!("3F.94.E2.AA.46.63.AA.78", seed_to_des("ABCD"));
}

#[test]
#[should_panic]
fn small_multiple_invalid_range() {
    println!(
        "{:?}",
        generate_multiple("2021-12-26", "2021-12-25", DEFAULT_SEED)
    );
}

#[test]
#[should_panic]
fn multiple_exceed_year() {
    println!(
        "{:?}",
        generate_multiple("2021-12-25", "2022-12-26", DEFAULT_SEED)
    );
}

#[test]
#[should_panic]
fn small_seed() {
    generate("1960-10-22", "ABC");
}

#[test]
#[should_panic]
fn large_seed() {
    generate("1960-10-22", "ABCABCABC");
}

#[test]
#[should_panic]
fn invalid_date() {
    generate("Phoenix dactylifera", DEFAULT_SEED);
}

#[test]
#[should_panic]
fn des_seed_short() {
    seed_to_des("ABC");
}
