pub fn concatenate_numbers(num1: u64, num2: u64) -> u64 {
    let num_digits_right_part = get_number_of_digits(num2);
    num1 * 10_u64.pow(num_digits_right_part) + num2
}

fn get_number_of_digits(num: u64) -> u32 {
    let num_digits = (num as f64).log10().floor() as u32 + 1;
    num_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_of_digits() {
        assert_eq!(get_number_of_digits(1234), 4);
    }

    #[test]
    fn test_concatenate_numbers() {
        let num1 = 123;
        let num2 = 45;
        let expected_result = 12345;
        assert_eq!(concatenate_numbers(num1, num2), expected_result);
    }
}
