use std::fs;

const NUM_DIGITS: usize = 50;
const NUM_NUMBERS: usize = 100;

fn read_numbers(path: &str) -> String {
    fs::read_to_string(path).unwrap()
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

fn get_sum_by_digit(num_array: [[u16; NUM_DIGITS]; NUM_NUMBERS]) -> [u16; NUM_DIGITS] {
    let mut sum_num_array = [0; NUM_DIGITS];
    for digit in 0..NUM_DIGITS {
        sum_num_array[digit] = num_array.iter().map(|num| num[digit]).sum::<u16>();
    }
    sum_num_array
}

fn propagate_sums(mut sum_num_array: [u16; NUM_DIGITS]) -> Vec<u16> {
    sum_num_array.reverse();
    let mut quotient = 0;
    let mut my_number = vec![];
    for (digit, value) in sum_num_array.iter().enumerate() {
        if digit + 1 == NUM_DIGITS {
            let mut remainder = *value + quotient;
            while remainder > 10 {
                my_number.push(remainder % 10);
                remainder /= 10
            }
            my_number.push(remainder);
        } else {
            let with_carry_over = value + quotient;
            let remainder = with_carry_over % 10;
            quotient = with_carry_over / 10;
            my_number.push(remainder);
        }
    }
    my_number.reverse();
    my_number
}

fn main() {
    let input = read_numbers("input.txt");
    let num_array = create_num_array(input);
    let sum_by_digit = get_sum_by_digit(num_array);
    let result = propagate_sums(sum_by_digit);
    println!(
        "{:?}",
        &result.iter().map(|num| num.to_string()).collect::<String>()[0..10]
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_num_array() {
        let input = read_numbers("input.txt");
        let num_array = create_num_array(input);

        assert_eq!(num_array[0][11], 9);
        assert_eq!(num_array[22][21], 4);
        assert_eq!(num_array[92][38], 6);
    }

    #[test]
    fn test_get_sum_by_digit() {
        let expected = 476;
        let input = read_numbers("input.txt");
        let num_array = create_num_array(input);
        let result = get_sum_by_digit(num_array);

        assert_eq!(result[25], expected);
    }

    #[test]
    fn test_propagate_sums() {
        let input = read_numbers("input.txt");
        let num_array = create_num_array(input);
        let sum_by_digit = get_sum_by_digit(num_array);
        let mut result = propagate_sums(sum_by_digit);

        println!("{:?}", sum_by_digit);
        println!("{:?}", result);

        assert_eq!(result.pop().unwrap(), 2);
    }
}
