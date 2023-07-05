pub fn luhn(cc_number: &str) -> bool {
    let number = cc_number.replace(" ", "");

    if number.len() < 2 {
        return false;
    }

    if !number.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    let sum: u32 = number
       .chars()
       .rev()
       .enumerate()
       .map(|(index, digit)| {
           let mut value = digit.to_digit(10).unwrap();
           if index % 2 == 1 {
               value *= 2;
               if value > 9 {
                   value = value % 10 + 1;
               }
           }
           value
       })
       .sum();

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
