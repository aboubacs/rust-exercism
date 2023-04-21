/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for ch in code.chars().rev() {
        if ch.is_whitespace() {
            continue;
        }
        if !ch.is_digit(10) {
            return false;
        }
        let mut digit = ch.to_digit(10).unwrap();
        if count % 2 == 1 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        count += 1;
    }
    count > 1 && sum % 10 == 0
}
