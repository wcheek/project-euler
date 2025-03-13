const START: u64 = 1_000_000;

fn operate_on_even(num: u64) -> u64 {
    num / 2
}

fn operate_on_odd(num: u64) -> u64 {
    num * 3 + 1
}

fn get_chain_for(num: u64) -> Vec<u64> {
    let mut chain = vec![num];
    let mut current_value = num;
    while current_value > 1 {
        if current_value % 2 == 0 {
            current_value = operate_on_even(current_value);
        } else {
            current_value = operate_on_odd(current_value);
        }
        chain.push(current_value);
    }
    chain
}

fn main() {
    let mut chain_lengths = vec![];
    for num in (1..START).rev() {
        chain_lengths.push(get_chain_for(num).len());
    }
    let index_of_max = chain_lengths
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(ind, _)| ind)
        .unwrap();
    let bla: Vec<u64> = (1..START).rev().collect();
    println!("{:?}", bla[index_of_max])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_chain_for() {
        let result = get_chain_for(13);

        assert_ne!(result, Vec::from([13, 40, 20, 10, 5, 16, 8, 4, 2, 1]));
    }
}
