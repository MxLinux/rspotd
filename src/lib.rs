use block_modes::{BlockMode, Cbc};
use block_padding::ZeroPadding;
use des::Des;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::mem::replace;
use time::macros::format_description;
use time::Date;
use time::Duration;

// Create a date range using a start and end date
struct DateRange(Date, Date);
impl Iterator for DateRange {
    type Item = Date;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(replace(&mut self.0, next))
        } else {
            None
        }
    }
}

struct EncodedDES(String);
struct Seed(String);

#[derive(Debug, Deserialize, Serialize)]
struct PotD {
    valid_date: String,
    given_seed: String,
    padded_seed: String,
    potd: String,
}

impl PotD {
    fn new(date: &str, seed: &str) -> Result<PotD, Box<dyn Error>> {
        let given_seed = seed.to_string();
        let padded_seed = validate_seed(seed)?;
        let valid_date = validate_date(date)?.to_string();
        let potd = generate(date, seed)?;
        Ok(PotD {
            valid_date,
            given_seed,
            padded_seed,
            potd,
        })
    }
}

fn derive_from_input(date: &str, padded_seed: &str) -> String {
    use vals::{ALPHANUM, TABLE1, TABLE2};
    let fmt_date = validate_date(date).unwrap();
    // Split date in YYYY-MM-DD format by hypen into a Vector of strings
    let date_components: Vec<i32> = date
        .split('-')
        .map(|i| i.parse::<i32>().expect("Error parsing date string"))
        .collect();
    let year = date_components[0];
    // Convert year to a string to get last two chars, then cast to i32
    let year_trimmed = year.to_string()[2..].parse::<i32>().unwrap();
    let month = date_components[1];
    let day = date_components[2];
    let day_of_week = fmt_date.weekday().number_days_from_monday() as usize;
    let a: Vec<i32> = (0..8)
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
    let b: Vec<i32> = padded_seed.chars().map(|c| c as i32).collect();
    let first_eight: Vec<i32> = (0..8).map(|i| (a[i] + b[i]) % 36).collect();
    let sum_of_parts: i32 = first_eight.iter().sum();
    let mut c: Vec<i32> = Vec::from(first_eight);
    c.push(sum_of_parts % 36);
    let last_value = (c[8] % 6).pow(2) as f64;
    if (last_value - last_value.floor()) < 0.5 {
        c.push(last_value.floor() as i32)
    } else {
        c.push(last_value.ceil() as i32);
    }
    let d: Vec<i32> = (0..10)
        .map(|i| c[TABLE2[(c[8] % 6) as usize][i] as usize])
        .collect();
    let vec_a: Vec<i32> = padded_seed
        .chars()
        .enumerate()
        .map(|(i, c)| (c as i32 + d[i]) % 36)
        .collect();
    let vec_b: String = (0..10)
        .map(|i: i32| ALPHANUM[vec_a[i as usize] as usize])
        .collect();
    return vec_b;
}

fn pad_seed(seed: &str) -> String {
    let mut padded = seed.to_string();
    if seed.len() == 4 {
        let diff = format!("{}{}", &seed, &seed[0..2]);
        padded.push_str(&diff);
        return padded;
    }
    let diff: String = (0..10 - seed.len())
        .into_iter()
        .map(|i| seed.as_bytes()[i as usize] as char)
        .into_iter()
        .collect();
    padded.push_str(&diff);
    padded
}

fn validate_seed(seed: &str) -> Result<String, Box<dyn Error>> {
    use vals::DEFAULT_SEED;
    if seed == DEFAULT_SEED {
        return Ok(seed.to_string());
    } else if seed.len() < 4 || seed.len() > 8 {
        Err("Seed should be >= 4 and <= 8 characters long.")?;
    }
    return Ok(pad_seed(seed));
}

fn validate_date(date: &str) -> Result<Date, Box<dyn Error>> {
    let fmt = format_description!("[year]-[month]-[day]");
    let date_regex: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !date_regex.is_match(date) {
        Err("Invalid date format, must be YYYY-MM-DD")?;
    }
    let parsed_date = Date::parse(date, fmt)?;
    return Ok(parsed_date);
}

fn validate_range(date_begin: &str, date_end: &str) -> Result<bool, Box<dyn Error>> {
    // TODO: return Ok(DateRange) instead and move time validation to implementation
    let begin = validate_date(date_begin)?;
    let end = validate_date(date_end)?;
    if end - begin <= time::Duration::days(0) {
        Err("Invalid date range. Beginning date must occur before end date, and the values cannot be the same.")?;
    }
    if end - begin > Duration::days(365) {
        Err("Invalid date range. Official tooling does not allow a date range exceeding 1 year.")?;
    }
    return Ok(true);
}

/// Generate an ARRIS/Commscope modem password given a date and seed
///
/// # Examples
///
/// ## Using default seed
///
/// ```no_run
/// use rspotd::{generate, vals::DEFAULT_SEED};
///
/// generate("2021-12-25", DEFAULT_SEED).unwrap();
/// ```
///
/// ## Using custom seed
///
/// ```no_run
/// use rspotd::generate;
///
/// generate("2021-12-25", "ABCDEFGH").unwrap();
/// ```
pub fn generate(date: &str, seed: &str) -> Result<String, Box<dyn Error>> {
    // TODO: derive_from_input should accept a Date?
    // valid_date is also called in derive_from_input...
    validate_date(date)?;
    let valid_seed = validate_seed(seed)?;
    return Ok(derive_from_input(date, &valid_seed));
}

/// Generate a series of ARRIS/Commscope modem passwords given a start and end date and a seed
///
/// # Examples
///
/// ## Using default seed
///
/// ```no_run
/// use rspotd::{generate_multiple, vals::DEFAULT_SEED};
///
/// generate_multiple("2021-07-23", "2022-07-28", DEFAULT_SEED).unwrap();
/// ```
///
/// ## Using custom seed
///
/// ```no_run
/// use rspotd::generate_multiple;
///
/// generate_multiple("2021-07-23", "2022-07-28", "ABCDABCD").unwrap();
/// ```
pub fn generate_multiple(
    date_begin: &str,
    date_end: &str,
    seed: &str,
) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let begin = validate_date(date_begin)?;
    let end = validate_date(date_end)?;
    validate_range(date_begin, date_end)?;
    let date_range = DateRange(begin, end);
    let valid_seed = validate_seed(seed)?;
    let mut potd_map = BTreeMap::new();
    for date in date_range {
        let date_string = date.to_string();
        let potd = derive_from_input(&date_string, valid_seed.as_str());
        potd_map.insert(date_string, potd);
    }
    return Ok(potd_map);
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
/// seed_to_des("ASDF").unwrap();
/// ```
pub fn seed_to_des(seed: &str) -> Result<String, Box<dyn Error>> {
    use vals::{DEFAULT_DES, DEFAULT_SEED};
    if seed == DEFAULT_SEED {
        return Ok(DEFAULT_DES.to_string());
    }
    if seed.len() < 4 || seed.len() > 8 {
        Err("Seed should be >= 4 and <= 8 characters long.")?;
    }
    let key = [20, 157, 64, 213, 193, 46, 85, 2];
    let iv = [0, 0, 0, 0, 0, 0, 0, 0];
    type DesCbc = Cbc<Des, ZeroPadding>;
    let cipher = DesCbc::new_from_slices(&key, &iv).unwrap();
    let mut seed_buffer = [0u8; 8];
    seed_buffer[..seed.len()].copy_from_slice(seed.as_bytes());
    let encrypted_seed = cipher.encrypt(&mut seed_buffer, seed.len()).unwrap();
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
    Ok(seed_string)
}

#[cfg(test)]
mod tests;
pub mod vals;
