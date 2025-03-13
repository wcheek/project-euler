use std::fs;

fn read_numbers() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn create_num_array(input: String) -> [[u8; 50]; 100] {
    let mut num_array = [[0; 50]; 100];
    for (line_num, line) in input.lines().enumerate() {
        num_array[line_num] = line
            .chars()
            .map(|num_str| num_str.to_string().parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
    }
    num_array
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
    }
}
