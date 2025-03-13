use std::{collections::HashSet, error::Error, fs};

#[derive(PartialEq, Debug, Hash, Eq)]
enum AcceptableDirections {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

struct Coordinates {
    row_num: usize,
    col_num: usize,
}

fn get_acceptable_directions(coords: Coordinates) -> HashSet<AcceptableDirections> {
    let mut acceptable_directions: HashSet<AcceptableDirections> = HashSet::new();

    if coords.row_num > 3 {
        acceptable_directions.insert(AcceptableDirections::Up);
        if coords.col_num < 16 {
            acceptable_directions.insert(AcceptableDirections::UpRight);
        }
    }
    if coords.col_num < 16 {
        acceptable_directions.insert(AcceptableDirections::Right);
        if coords.row_num < 16 {
            acceptable_directions.insert(AcceptableDirections::DownRight);
        }
    }
    if coords.row_num < 16 {
        acceptable_directions.insert(AcceptableDirections::Down);
        if coords.col_num > 3 {
            acceptable_directions.insert(AcceptableDirections::DownLeft);
        }
    }
    if coords.col_num > 3 {
        acceptable_directions.insert(AcceptableDirections::Left);
        if coords.row_num > 3 {
            acceptable_directions.insert(AcceptableDirections::UpLeft);
        }
    }

    acceptable_directions
}

fn get_num_grid() -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let mut grid: Vec<Vec<u32>> = vec![];
    let input = fs::read_to_string("input.txt")?;
    for line in input.lines() {
        let mut this_row = vec![];
        for num in line.split_whitespace() {
            let num = num.parse::<u32>()?;
            this_row.push(num);
        }
        grid.push(this_row);
    }
    Ok(grid)
}

fn main() {
    if let Ok(grid) = get_num_grid() {
        for (row_num, row) in grid.iter().enumerate() {
            for (col_num, col) in row.iter().enumerate() {
                let acceptable_directions =
                    get_acceptable_directions(Coordinates { row_num, col_num });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_num_grid() {
        let grid = get_num_grid().unwrap();

        assert_eq!(grid.len(), 20);
        assert_eq!(grid[0].len(), 20);

        assert_eq!(grid[0][0], 8);
        assert_eq!(grid[19][0], 1);
        assert_eq!(grid[9][19], 95);
    }

    #[test]
    fn test_get_acceptable_directions() {
        let acceptable_directions = get_acceptable_directions(Coordinates {
            row_num: 0,
            col_num: 0,
        });

        assert!(!acceptable_directions.is_empty());
        assert_eq!(
            acceptable_directions,
            HashSet::from([
                AcceptableDirections::Right,
                AcceptableDirections::Down,
                AcceptableDirections::DownRight,
            ])
        );

        let acceptable_directions = get_acceptable_directions(Coordinates {
            row_num: 19,
            col_num: 19,
        });

        assert!(!acceptable_directions.is_empty());
        assert_eq!(
            acceptable_directions,
            HashSet::from([
                AcceptableDirections::Up,
                AcceptableDirections::UpLeft,
                AcceptableDirections::Left,
            ])
        );

        let acceptable_directions = get_acceptable_directions(Coordinates {
            row_num: 10,
            col_num: 10,
        });

        assert!(!acceptable_directions.is_empty());
        assert_eq!(
            acceptable_directions,
            HashSet::from([
                AcceptableDirections::Right,
                AcceptableDirections::Down,
                AcceptableDirections::DownRight,
                AcceptableDirections::Up,
                AcceptableDirections::UpLeft,
                AcceptableDirections::Left,
                AcceptableDirections::DownLeft,
                AcceptableDirections::UpRight
            ])
        );
    }
}
