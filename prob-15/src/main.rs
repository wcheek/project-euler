use num::BigInt;

fn calculate_factorial_of(num: u64) -> BigInt {
    let mut result: BigInt = num.into();
    for i in (1..num).rev() {
        result *= i;
    }
    result
}

fn main() {
    //40!/(20!*20!)
    let solution =
        calculate_factorial_of(40) / (calculate_factorial_of(20) * calculate_factorial_of(20));

    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_factorial_of() {
        let result = calculate_factorial_of(5);
        assert_eq!(result, 120.into());
    }
}
