use std::{collections::HashSet, error::Error, fs};

#[derive(PartialEq, Debug, Hash, Eq)]
enum AcceptableDirection {
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

fn get_acceptable_directions(coords: Coordinates) -> HashSet<AcceptableDirection> {
    let mut acceptable_directions: HashSet<AcceptableDirection> = HashSet::new();
    const START_EDGE: usize = 2;
    const END_EDGE: usize = 17;

    if coords.row_num > START_EDGE {
        acceptable_directions.insert(AcceptableDirection::Up);
        if coords.col_num < END_EDGE {
            acceptable_directions.insert(AcceptableDirection::UpRight);
        }
    }
    if coords.col_num < END_EDGE {
        acceptable_directions.insert(AcceptableDirection::Right);
        if coords.row_num < END_EDGE {
            acceptable_directions.insert(AcceptableDirection::DownRight);
        }
    }
    if coords.row_num < END_EDGE {
        acceptable_directions.insert(AcceptableDirection::Down);
        if coords.col_num > START_EDGE {
            acceptable_directions.insert(AcceptableDirection::DownLeft);
        }
    }
    if coords.col_num > START_EDGE {
        acceptable_directions.insert(AcceptableDirection::Left);
        if coords.row_num > START_EDGE {
            acceptable_directions.insert(AcceptableDirection::UpLeft);
        }
    }

    acceptable_directions
}

fn get_product(
    grid: &Vec<Vec<u32>>,
    coordinates: Coordinates,
    for_direction: AcceptableDirection,
) -> u32 {
    match for_direction {
        AcceptableDirection::Up => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num - 1][coordinates.col_num]
                * grid[coordinates.row_num - 2][coordinates.col_num]
                * grid[coordinates.row_num - 3][coordinates.col_num]
        }
        AcceptableDirection::UpRight => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num - 1][coordinates.col_num + 1]
                * grid[coordinates.row_num - 2][coordinates.col_num + 2]
                * grid[coordinates.row_num - 3][coordinates.col_num + 3]
        }
        AcceptableDirection::Right => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num][coordinates.col_num + 1]
                * grid[coordinates.row_num][coordinates.col_num + 2]
                * grid[coordinates.row_num][coordinates.col_num + 3]
        }
        AcceptableDirection::DownRight => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num + 1][coordinates.col_num + 1]
                * grid[coordinates.row_num + 2][coordinates.col_num + 2]
                * grid[coordinates.row_num + 3][coordinates.col_num + 3]
        }
        AcceptableDirection::Down => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num + 1][coordinates.col_num]
                * grid[coordinates.row_num + 2][coordinates.col_num]
                * grid[coordinates.row_num + 3][coordinates.col_num]
        }
        AcceptableDirection::DownLeft => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num + 1][coordinates.col_num - 1]
                * grid[coordinates.row_num + 2][coordinates.col_num - 2]
                * grid[coordinates.row_num + 3][coordinates.col_num - 3]
        }
        AcceptableDirection::Left => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num][coordinates.col_num - 1]
                * grid[coordinates.row_num][coordinates.col_num - 2]
                * grid[coordinates.row_num][coordinates.col_num - 3]
        }
        AcceptableDirection::UpLeft => {
            grid[coordinates.row_num][coordinates.col_num]
                * grid[coordinates.row_num - 1][coordinates.col_num - 1]
                * grid[coordinates.row_num - 2][coordinates.col_num - 2]
                * grid[coordinates.row_num - 3][coordinates.col_num - 3]
        }
    }
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
    let mut products: Vec<u32> = vec![];
    if let Ok(grid) = get_num_grid() {
        for (row_num, row) in grid.iter().enumerate() {
            for (col_num, _col) in row.iter().enumerate() {
                for direction in get_acceptable_directions(Coordinates { row_num, col_num }) {
                    products.push(get_product(
                        &grid,
                        Coordinates { row_num, col_num },
                        direction,
                    ))
                }
            }
        }
    }
    println!("{}", products.iter().max().unwrap())
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
                AcceptableDirection::Right,
                AcceptableDirection::Down,
                AcceptableDirection::DownRight,
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
                AcceptableDirection::Up,
                AcceptableDirection::UpLeft,
                AcceptableDirection::Left,
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
                AcceptableDirection::Right,
                AcceptableDirection::Down,
                AcceptableDirection::DownRight,
                AcceptableDirection::Up,
                AcceptableDirection::UpLeft,
                AcceptableDirection::Left,
                AcceptableDirection::DownLeft,
                AcceptableDirection::UpRight
            ])
        );
    }

    #[test]
    fn test_get_product() {
        let grid = get_num_grid().unwrap();

        let product = get_product(
            &grid,
            Coordinates {
                row_num: 0,
                col_num: 0,
            },
            AcceptableDirection::Right,
        );
        assert_eq!(product, 34144);

        let product = get_product(
            &grid,
            Coordinates {
                row_num: 0,
                col_num: 0,
            },
            AcceptableDirection::DownRight,
        );
        assert_eq!(product, 279496);

        let product = get_product(
            &grid,
            Coordinates {
                row_num: 0,
                col_num: 0,
            },
            AcceptableDirection::Down,
        );
        assert_eq!(product, 1_651_104);
    }
}
