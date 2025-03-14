use num::BigInt;

// 2^10*2^10*2^10
fn main() {
    let num: BigInt = 2.into();
    let power = num.pow(1000);
    let sum: u32 = power
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();
    println!("{}", sum);
}
