use block_modes::{BlockMode, Cbc};
use block_padding::{Padding, ZeroPadding};
use chrono::format::strftime::StrftimeItems;
use chrono::{Datelike, Duration, NaiveDate};
use des::Des;
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

/// Default seed used by ARRIS/CommScope, represents the string "MPSJKMDHAI"
pub static DEFAULT_SEED: &str = "MPSJKMDHAI";
static TABLE1: [[i32; 5]; 7] = [
    [15, 15, 24, 20, 24],
    [13, 14, 27, 32, 10],
    [29, 14, 32, 29, 24],
    [23, 32, 24, 29, 29],
    [14, 29, 10, 21, 29],
    [34, 27, 16, 23, 30],
    [14, 22, 24, 17, 13],
];
static TABLE2: [[i32; 10]; 6] = [
    [0, 1, 2, 9, 3, 4, 5, 6, 7, 8],
    [1, 4, 3, 9, 0, 7, 8, 2, 5, 6],
    [7, 2, 8, 9, 4, 1, 6, 0, 3, 5],
    [6, 3, 5, 9, 1, 8, 2, 7, 4, 0],
    [4, 7, 0, 9, 5, 2, 3, 1, 8, 6],
    [5, 6, 1, 9, 8, 0, 4, 3, 2, 7],
];
static ALPHANUM: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn first_pass(date: &str) -> Vec<i32> {
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    // Split date in YYYY-MM-DD format by hypen into a Vector of strings
    let date_components: Vec<i32> = date
        .split('-')
        .map(|i| i.parse::<i32>().expect("Error parsing date string"))
        .collect();
    let year = date_components[0];
    // Convert year to a string to get last two chars, then cast to i32
    let year_trimmed = &year.to_string()[2..].parse::<i32>().unwrap();
    let month = date_components[1];
    let day = date_components[2];
    let day_of_week = naive_date.weekday().num_days_from_monday() as usize;
    let result: Vec<i32> = (0..8)
        .map(|i| match i {
            0..=4 => TABLE1[day_of_week][i],
            5 => day,
            6 => {
                if ((year_trimmed + month) - day) < 0 {
                    (((year_trimmed + month) - day) + 36) % 36
                } else {
                    ((year_trimmed + month) - day) % 36
                }
            }
            _ => (((3 + ((year_trimmed + month) % 12)) * day) % 37) % 36,
        })
        .collect();
    return result;
}

fn second_pass(padded_seed: &str) -> Vec<i32> {
    let result: Vec<i32> = padded_seed.chars().map(|c| c as i32).collect();
    return result;
}

fn third_pass(first_result: Vec<i32>, second_result: Vec<i32>) -> Vec<i32> {
    let first_eight: Vec<i32> = (0..8)
        .map(|i| (first_result[i] + second_result[i]) % 36)
        .collect();
    let sum_of_parts: i32 = first_eight.iter().sum();
    let mut result: Vec<i32> = Vec::from(first_eight);
    result.push(sum_of_parts % 36);
    let _last_value = (result[8] % 6).pow(2) as f64;
    if (_last_value - _last_value.floor()) < 0.5 {
        result.push(_last_value.floor() as i32)
    } else {
        result.push(_last_value.ceil() as i32);
    }
    return result;
}

fn fourth_pass(third_result: Vec<i32>) -> Vec<i32> {
    let result: Vec<i32> = (0..10)
        .map(|i| third_result[TABLE2[(third_result[8] % 6) as usize][i] as usize])
        .collect();
    return result;
}

fn fifth_pass(padded_seed: &str, fourth_result: Vec<i32>) -> Vec<i32> {
    let result: Vec<i32> = padded_seed
        .chars()
        .enumerate()
        .map(|(i, c)| (c as i32 + fourth_result[i]) % 36)
        .collect();
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
        panic!(
            "Invalid date range. Official tooling does not allow a date range exceeding 1 year."
        );
    }
}

/// Generate an ARRIS/Commscope modem password given a date and seed
///
/// # Examples
///
/// ## Using default seed
///
/// ```no_run
/// use rspotd::{DEFAULT_SEED, generate};
///
/// generate("2021-12-25", DEFAULT_SEED);
/// ```
///
/// ## Using custom seed
///
/// ```no_run
/// use rspotd::generate;
///
/// generate("2021-12-25", "ABCDEFGH");
/// ```
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

/// Generate a series of ARRIS/Commscope modem passwords given a start and end date and a seed
///
/// # Examples
///
/// ## Using default seed
///
/// ```no_run
/// use rspotd::{DEFAULT_SEED, generate_multiple};
///
/// generate_multiple("2021-07-23", "2022-07-28", DEFAULT_SEED);
/// ```
///
/// ## Using custom seed
///
/// ```no_run
/// use rspotd::generate_multiple;
///
/// generate_multiple("2021-07-23", "2022-07-28", "ABCDABCD");
/// ```
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
        potd_map.insert(date_string, potd);
    }
    return potd_map;
}

/// Creates the required dot-delimited hex string correlating to the provided seed DES-encrypted.
///
/// The value provided by this function can be added to a modem configuration file, and a modem with this config
/// will subsequently respond to a password of the day generated by the same seed used to create the DES-encrypted value.
///
/// Note, you cannot configure your modem from the subscriber-side. Your modem downloads its configuration
/// typically via TFTP from a server inside your ISP's infrastructure.
///
/// The default ARRIS/CommScope value is provided if you use the default seed. I have not yet figured out
/// how they generate the DES-encrypted value with a seed that exceeds a block size of 8, so I have to hardcode the value.
///
/// In the official tooling, the DES value is not provided if you select the "Use default seed" checkbox, and
/// as far as I can tell, the software maintains a firm understanding throughout that your seed is between
/// 4 and 8 characters.
///
/// Only one such value will exist for any number of passwords of a given seed; the modem infers the seed from this value.
///
/// ## Example
///
/// ```no_run
/// use rspotd::seed_to_des;
///
/// seed_to_des("ASDF");
/// ```
pub fn seed_to_des(seed: &str) -> String {
    if seed.len() < 4 || seed.len() > 8 {
        panic!("Seed should be >= 4 and <= 8 characters long.");
    }
    let default_des: String = "DB.B5.CB.D6.11.17.D6.EB".to_string();
    if seed == DEFAULT_SEED {
        default_des
    } else {
        let key = [20, 157, 64, 213, 193, 46, 85, 2];
        let iv = [0, 0, 0, 0, 0, 0, 0, 0];
        type DesCbc = Cbc<Des, ZeroPadding>;
        let cipher = DesCbc::new_from_slices(&key, &iv).unwrap();
        let mut seed_buffer = [0u8; 8];
        let seed_as_bytes = seed.as_bytes();
        let seed_len = seed.len();
        seed_buffer[..seed_len].copy_from_slice(seed_as_bytes);
        let padded_seed = ZeroPadding::pad(&mut seed_buffer, seed_len, 8).unwrap();
        let encrypted_seed = cipher.encrypt(padded_seed, seed_len).unwrap();
        let seed_string: String = encrypted_seed
            .iter()
            .map(|i| {
                if i == &encrypted_seed[7] {
                    format!("{:X}", i)
                } else {
                    format!("{:X}.", i)
                }
            })
            .collect();
        seed_string
    }
}

#[cfg(test)]
mod tests;
