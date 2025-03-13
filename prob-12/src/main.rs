fn find_divisors(for_number: u64) -> Vec<u64> {
    let mut divisors = vec![];
    if for_number == 1 {
        divisors.push(1)
    } else {
        for i in 1..=for_number.isqrt() {
            if for_number % i == 0 {
                divisors.push(i);
            }
        }
        for i in (1..=for_number.isqrt()).rev() {
            if for_number % i == 0 {
                divisors.push(for_number / i);
            }
        }
    }
    divisors
}

fn main() {
    let mut num_divisors = 0;
    let mut current_triangle_num = 1;
    let mut ind = 2;
    while num_divisors < 500 {
        current_triangle_num += ind;
        let divisors = find_divisors(current_triangle_num);
        num_divisors = divisors.len();
        ind += 1;
    }
    println!("{}", current_triangle_num);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_divisors() {
        let divisors = find_divisors(1);
        assert_eq!(divisors, Vec::from([1]));

        let divisors = find_divisors(3);
        assert_eq!(divisors, Vec::from([1, 3]));

        let divisors = find_divisors(6);
        assert_eq!(divisors, Vec::from([1, 2, 3, 6]));

        let divisors = find_divisors(10);
        assert_eq!(divisors, Vec::from([1, 2, 5, 10]));

        let divisors = find_divisors(15);
        assert_eq!(divisors, Vec::from([1, 3, 5, 15]));

        let divisors = find_divisors(21);
        assert_eq!(divisors, Vec::from([1, 3, 7, 21]));

        let divisors = find_divisors(28);
        assert_eq!(divisors, Vec::from([1, 2, 4, 7, 14, 28]));
    }
}
