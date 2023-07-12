

// https://adventofcode.com/2020/day/3

use std::{ops::Index, string::ParseError};

// // Returns a Vector<String> representing the top crate left on each stack
pub fn count_trees_part_one(grid: &Vec<String>) -> i32 {
    let mut trees_count = 0;
    let mut coor = (0, 0);
    match grid.get(0) {
        Some(first_row) => { 
            let row_length = first_row.len();
            loop {
                coor.0 += 3;
                coor.1 += 1;
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

fn highlight_coordinate(grid: &Vec<&str>, coor: (usize, usize)) -> Vec<String> {
    let mut highlighted_grid = Vec::new();
    for (i, string) in grid.iter().enumerate() {
        if i == coor.1 {
            let mut highlighted = String::new();
            for (j, c) in string.chars().enumerate() {
                if j == coor.0 % 11 {
                    highlighted.push('*');
                }
                highlighted.push(c);
            }
            highlighted_grid.push(highlighted)
        } else {
            highlighted_grid.push(String::from(*string));
        }
    }
    highlighted_grid
}

#[test]
fn test_count_trees_part_one() {
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
    assert_eq!(count_trees_part_one(&grid), 7); 
}

// Give me the forecast for the weather this weekend
// Using a public API for weather
