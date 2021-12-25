use chrono::{NaiveDate, Datelike};

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
    let date_components: Vec<&str> = date.split('-').collect();
    let year = date_components[0].parse::<i32>().unwrap();
    let year_trimmed = &year.to_string()[2..].parse::<i32>().unwrap();
    let month = date_components[1].parse::<u32>().unwrap();
    let day = date_components[2].parse::<u32>().unwrap();
    let day_of_week = naive_date.weekday().num_days_from_monday() as usize;
    let mut result = Vec::new();
    for i in 0..5 {
        result.push(TABLE1[day_of_week][i])
    }
    result.push(day as i32);
    if ((*year_trimmed as i32 + month as i32) - day as i32) < 0 {
        result.push((((*year_trimmed + month as i32) - day as i32) + 36) % 36);
    }
    else {
        result.push(((*year_trimmed + month as i32) - day as i32) % 36)
    }
    result.push((((3 + ((*year_trimmed + month as i32) % 12)) * day as i32) % 37) % 36);
    return result;
}

fn second_pass(padded_seed: &str) -> Vec<i32> {
    let mut result = Vec::new();
    for c in padded_seed.chars() {
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
    // TODO: Check for seed regex
    let mut owned_seed = seed.to_owned();
    if seed.len() < 10 {
        let len_diff: i32 = 10 - seed.len() as i32;
        for i in 0..len_diff {
            owned_seed.push(seed.as_bytes()[i as usize] as char)
        }
    }
    return owned_seed;
}

pub fn generate(date: &str, seed: &str) -> String {
    let mut owned_seed = seed.to_owned();
    let mut potd_vec: Vec<char> = Vec::new();
    if seed != DEFAULT_SEED {
        owned_seed = validate_seed(seed);
    }
    let fifth_result = derive_from_input(date, &owned_seed);
    for i in 0..10 {
        potd_vec.push(ALPHANUM[fifth_result[i as usize] as usize])
    }
    let potd: String = potd_vec.into_iter().collect();
    return potd;
}

#[cfg(test)]
mod tests {
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
}