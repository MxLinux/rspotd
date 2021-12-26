use chrono::{Datelike, Duration, NaiveDate};
use chrono::format::strftime::StrftimeItems;
use regex::Regex;
use std::collections::HashMap;
use std::mem;

// Create a date range using a start and end date
struct DateRange(NaiveDate, NaiveDate);
impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

static DEFAULT_SEED: &str = "MPSJKMDHAI";
static TABLE1: [[i32; 5]; 7] = [
    [15, 15, 24, 20, 24],
    [13, 14, 27, 32, 10],
    [29, 14, 32, 29, 24],
    [23, 32, 24, 29, 29],
    [14, 29, 10, 21, 29],
    [34, 27, 16, 23, 30],
    [14, 22, 24, 17, 13]
];
static TABLE2: [[i32; 10]; 6] = [
    [0, 1, 2, 9, 3, 4, 5, 6, 7, 8],
    [1, 4, 3, 9, 0, 7, 8, 2, 5, 6],
    [7, 2, 8, 9, 4, 1, 6, 0, 3, 5],
    [6, 3, 5, 9, 1, 8, 2, 7, 4, 0],
    [4, 7, 0, 9, 5, 2, 3, 1, 8, 6],
    [5, 6, 1, 9, 8, 0, 4, 3, 2, 7]
];
static ALPHANUM: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8',
    '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];


fn first_pass(date: &str) -> Vec<i32> {
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    // Split date in YYYY-MM-DD format by hypen into a Vector of strings
    let date_components: Vec<&str> = date.split('-').collect();
    let year = date_components[0].parse::<i32>().unwrap();
    // Convert year to a string to get last two chars, then cast to i32
    let year_trimmed = &year.to_string()[2..].parse::<i32>().unwrap();
    let month = date_components[1].parse::<i32>().unwrap();
    let day = date_components[2].parse::<i32>().unwrap();
    let day_of_week = naive_date.weekday().num_days_from_monday() as usize;
    let mut result = Vec::new();
    for i in 0..5 {
        result.push(TABLE1[day_of_week][i])
    }
    result.push(day);
    if ((year_trimmed + month) - day) < 0 {
        result.push((((year_trimmed + month) - day) + 36) % 36);
    }
    else {
        result.push(((year_trimmed + month) - day) % 36)
    }
    result.push((((3 + ((year_trimmed + month) % 12)) * day) % 37) % 36);
    return result;
}

fn second_pass(padded_seed: &str) -> Vec<i32> {
    let mut result = Vec::new();
    for c in padded_seed.chars() {
        // Get the integer representation of each character
        let seed_char_code: i32 = c as i32;
        let char_mod: i32 = seed_char_code % 36;
        result.push(char_mod);
    }
    return result;
}

fn third_pass(first_result: Vec<i32>, second_result: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..8 {
        let mod_value: i32 = (first_result[i] + second_result[i]) % 36;
        result.push(mod_value);
    }
    let sum_of_parts: i32 = result.iter().sum();
    result.push(sum_of_parts % 36);
    let _last_value = (result[8] % 6).pow(2) as f64;
    if (_last_value - _last_value.floor()) < 0.5 {
        result.push(_last_value.floor() as i32)
    }
    else {
        result.push(_last_value.ceil() as i32);
    }
    return result;
}

fn fourth_pass(third_result: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..10 {
        let value = third_result[TABLE2[(third_result[8] % 6) as usize][i] as usize];
        result.push(value);
    }
    return result;
}

fn fifth_pass(padded_seed: &str, fourth_result: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for (i, c) in padded_seed.chars().enumerate() {
        let value = (c as i32 + fourth_result[i]) % 36;
        result.push(value);
    }
    return result;
}

fn derive_from_input(date: &str, padded_seed: &str) -> Vec<i32> {
    let first_result = first_pass(date);
    let second_result = second_pass(padded_seed);
    let third_result = third_pass(first_result, second_result);
    let fourth_result = fourth_pass(third_result);
    let fifth_result = fifth_pass(padded_seed, fourth_result);
    return fifth_result;
}

fn validate_seed(seed: &str) -> String {
    // seed must be 4-8 characters
    if seed.len() < 4 || seed.len() > 8 {
        panic!("Seed should be >= 4 and <= 8 characters long.")
    }
    let mut owned_seed = seed.to_owned();
    if seed.len() < 10 {
        let len_diff: i32 = 10 - seed.len() as i32;
        for i in 0..len_diff {
            owned_seed.push(seed.as_bytes()[i as usize] as char)
        }
    }
    return owned_seed;
}

fn validate_date(date: &str) {
    let date_regex: regex::Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !date_regex.is_match(date) {
        panic!("Invalid date format, must be YYYY-MM-DD")
    }
}

fn validate_range(date_begin: &str, date_end: &str) {
    let naive_begin = NaiveDate::parse_from_str(date_begin, "%Y-%m-%d").unwrap();
    let naive_end = NaiveDate::parse_from_str(date_end, "%Y-%m-%d").unwrap();
    if naive_end.signed_duration_since(naive_begin) <= Duration::zero() {
        panic!("Invalid date range. Beginning date must occur before end date, and the values cannot be the same.");
    }
    if naive_end - naive_begin > Duration::days(365) {
        panic!("Invalid date range. Official tooling does not allow a date range exceeding 1 year.");
    }
}

pub fn generate(date: &str, seed: &str) -> String {
    validate_date(date);
    let mut owned_seed = seed.to_owned();
    if seed != DEFAULT_SEED {
        owned_seed = validate_seed(seed);
    }
    let fifth_result = derive_from_input(date, &owned_seed);
    let mut potd_char_vec: Vec<char> = Vec::new();
    for i in 0..10 {
        potd_char_vec.push(ALPHANUM[fifth_result[i as usize] as usize]);
    }
    let potd: String = potd_char_vec.into_iter().collect();
    return potd;
}

pub fn generate_multiple(date_begin: &str, date_end: &str, seed: &str) -> HashMap<String, String> {
    validate_date(date_begin);
    validate_date(date_end);
    validate_range(date_begin, date_end);
    let naive_begin = NaiveDate::parse_from_str(date_begin, "%Y-%m-%d").unwrap();
    let naive_end = NaiveDate::parse_from_str(date_end, "%Y-%m-%d").unwrap();
    let date_range = DateRange(naive_begin, naive_end);
    let mut owned_seed = seed.to_owned();
    if seed != DEFAULT_SEED {
        owned_seed = validate_seed(seed);
    }
    let mut potd_map = HashMap::new();
    for date in date_range {
        let format = StrftimeItems::new("%Y-%m-%d");
        let date_string = date.format_with_items(format).to_string();
        let fifth_result = derive_from_input(&date_string, &owned_seed);
        let mut potd_char_vec: Vec<char> = Vec::new();
        for i in 0..10 {
            potd_char_vec.push(ALPHANUM[fifth_result[i as usize] as usize]);
        }
        let potd: String = potd_char_vec.into_iter().collect();
        potd_map.insert(
            date_string,
            potd,
        );
    }
    return potd_map;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn default_seed() {
        assert_eq!(super::generate("2021-12-25", super::DEFAULT_SEED), "ZCARK8TPK5");
    }
    #[test]
    fn custom_seed() {
        assert_eq!(super::generate("2021-12-25", "1122AABB"), "FEWNX8OS0O");
    }
    #[test]
    fn different_date() {
        assert_eq!(super::generate("1960-10-22", super::DEFAULT_SEED), "WGAR88TPKS");
    }
    #[test]
    fn small_multiple() {
        let mut comparison_map: HashMap<String, String> = HashMap::new();
        let date_begin: String = "2021-12-25".to_string();
        let date_end: String = "2021-12-26".to_string();
        let seed_begin: String = "ZCARK8TPK5".to_string();
        let seed_end: String = "ZOU3MLLZO4".to_string();
        comparison_map.insert(date_begin, seed_begin);
        comparison_map.insert(date_end, seed_end);
        assert_eq!(super::generate_multiple("2021-12-25", "2021-12-26", super::DEFAULT_SEED), comparison_map);
    }
    #[test]
    #[should_panic]
    fn small_multiple_invalid_range() {
        println!("{:?}", super::generate_multiple("2021-12-26", "2021-12-25", super::DEFAULT_SEED));
    }
    #[test]
    #[should_panic]
    fn multiple_exceed_year() {
        println!("{:?}", super::generate_multiple("2021-12-25", "2022-12-26", super::DEFAULT_SEED));
    }
    #[test]
    #[should_panic]
    fn small_seed() {
        super::generate("1960-10-22", "ABC");
    }
    #[test]
    #[should_panic]
    fn large_seed() {
        super::generate("1960-10-22", "ABCABCABC");
    }
    #[test]
    #[should_panic]
    fn invalid_date() {
        super::generate("Phoenix dactylifera", super::DEFAULT_SEED);
    }
}