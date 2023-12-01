

// https://adventofcode.com/2020/day/3

// // Returns a Vector<String> representing the top crate left on each stack
pub fn count_trees(grid: &Vec<String>, slope: (usize, usize)) -> usize {
    let mut trees_count = 0;
    let mut coor = (0, 0);
    match grid.get(0) {
        Some(first_row) => { 
            let row_length = first_row.len();
            loop {
                coor.0 += slope.0;
                coor.1 += slope.1;
                match grid.get(coor.1) {
                    Some(row) => {
                        let spot_idx = coor.0 % row_length;
                        match row.chars().nth(spot_idx) {
                            Some(spot) => {
                                if spot == '#' {
                                    trees_count += 1;
                                }
                            }
                            None => {
                                panic!("Shouldn't get here");
                            }
                        }
                    }
                    None => {
                        return trees_count;
                    }
                }
            }
        }
        None => {
            panic!("Expected grid to have at least one row")
        }
    }
}

#[test]
fn test_count_trees_part_one() {
    let grid: Vec<String> = vec![
        String::from("..##......."),
        String::from("#...#...#.."),
        String::from(".#....#..#."),
        String::from("..#.#...#.#"),
        String::from(".#...##..#."),
        String::from("..#.##....."),
        String::from(".#.#.#....#"),
        String::from(".#........#"),
        String::from("#.##...#..."),
        String::from("#...##....#"),
        String::from(".#..#...#.#")
    ];
    assert_eq!(count_trees(&grid, (3, 1)), 7); 
}

#[test]
fn test_count_trees_part_two() {
    let grid = vec![
        String::from("..##......."),
        String::from("#...#...#.."),
        String::from(".#....#..#."),
        String::from("..#.#...#.#"),
        String::from(".#...##..#."),
        String::from("..#.##....."),
        String::from(".#.#.#....#"),
        String::from(".#........#"),
        String::from("#.##...#..."),
        String::from("#...##....#"),
        String::from(".#..#...#.#")
    ];
    assert_eq!(count_trees(&grid, (1, 1)), 2); 
    assert_eq!(count_trees(&grid, (3, 1)), 7); 
    assert_eq!(count_trees(&grid, (5, 1)), 3); 
    assert_eq!(count_trees(&grid, (7, 1)), 4); 
    assert_eq!(count_trees(&grid, (1, 2)), 2); 
}

// Give me the forecast for the weather this weekend
// Using a public API for weather
