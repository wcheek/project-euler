use std::fs;

const NUM_DIGITS: usize = 50;
const NUM_NUMBERS: usize = 100;

fn read_numbers() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn create_num_array(input: String) -> [[u16; NUM_DIGITS]; NUM_NUMBERS] {
    let mut num_array = [[0; NUM_DIGITS]; NUM_NUMBERS];
    for (line_num, line) in input.lines().enumerate() {
        num_array[line_num] = line
            .chars()
            .map(|num_str| num_str.to_string().parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .try_into()
            .unwrap();
    }
    num_array
}

fn get_sum_by_digit(num_array: [[u16; NUM_DIGITS]; NUM_NUMBERS]) -> [u16; NUM_NUMBERS] {
    let mut sum_num_array = [0; NUM_NUMBERS];
    for digit in 0..NUM_DIGITS {
        let mut total_for_digit = 0;
        for num in 0..NUM_NUMBERS {
            total_for_digit += num_array[num][digit];
        }
        sum_num_array[digit] = total_for_digit;
    }
    sum_num_array
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_num_array() {
        let input = read_numbers();
        let num_array = create_num_array(input);

        assert_eq!(num_array[0][11], 9);
        assert_eq!(num_array[22][21], 4);
        assert_eq!(num_array[92][38], 6);
    }

    #[test]
    fn test_get_sum_by_digit() {
        let expected = 476;
        let input = read_numbers();
        let num_array = create_num_array(input);
        let result = get_sum_by_digit(num_array);

        assert_eq!(result[25], expected);
    }
}
