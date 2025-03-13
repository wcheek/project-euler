use std::fs;

fn get_num_grid() -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![];
    if let Ok(input) = fs::read_to_string("input.txt") {
        for line in input.lines() {
            let mut this_row = vec![];
            for num in line.split_whitespace() {
                if let Ok(num) = num.parse::<u32>() {
                    this_row.push(num);
                } else {
                    panic!("Could not parse num");
                }
            }
            grid.push(this_row);
        }
    } else {
        panic!("Could not read input");
    }
    grid
}

fn main() {
    let grid = get_num_grid();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_num_grid() {
        let grid = get_num_grid();

        assert_eq!(grid.len(), 20);
        assert_eq!(grid[0].len(), 20);

        assert_eq!(grid[0][0], 8);
        assert_eq!(grid[19][0], 1);
        assert_eq!(grid[9][19], 95);
    }
}
