#![feature(generators, generator_trait)]

use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq)]
enum PositionState {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn count_visible_occupied(
    seating_layout: &[Vec<PositionState>],
    height: usize,
    width: usize,
    x: usize,
    y: usize,
) -> usize {
    let mut counter = 0;
    let (x, y, height, width) = (x as isize, y as isize, height as isize, width as isize);

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for direction in &directions {
        let mut xx = x;
        let mut yy = y;
        loop {
            xx += direction.0;
            yy += direction.1;
            if xx < 0 || yy < 0 || xx >= height || yy >= width {
                break;
            }
            if seating_layout[xx as usize][yy as usize] == PositionState::EmptySeat {
                break;
            }
            if seating_layout[xx as usize][yy as usize] == PositionState::OccupiedSeat {
                counter += 1;
                break;
            }
        }
    }
    counter
}

#[allow(dead_code)]
fn print_layout(layout: &[Vec<PositionState>]) {
    for row in layout {
        for state in row {
            let letter = match state {
                PositionState::OccupiedSeat => '#',
                PositionState::EmptySeat => 'L',
                PositionState::Floor => '.',
            };
            print!("{}", letter);
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut cur_layout = vec![];
    for line in stdin.lock().lines() {
        let mut row_layout = vec![];
        for letter in line.unwrap().chars() {
            let state = match letter {
                'L' => PositionState::EmptySeat,
                '#' => PositionState::OccupiedSeat,
                '.' => PositionState::Floor,
                letter => panic!(format!("invalid position state '{}'!", letter)),
            };
            row_layout.push(state);
        }
        cur_layout.push(row_layout);
    }
    let height = cur_layout.len();
    let width = cur_layout[0].len();

    let mut new_layout = cur_layout.clone();

    loop {
        let mut change_flag = false;
        for i in 0..height {
            for j in 0..width {
                let new_state = match cur_layout[i][j] {
                    PositionState::Floor => PositionState::Floor,
                    PositionState::EmptySeat => {
                        if count_visible_occupied(&cur_layout, height, width, i, j) == 0 {
                            change_flag = true;
                            PositionState::OccupiedSeat
                        } else {
                            PositionState::EmptySeat
                        }
                    }
                    PositionState::OccupiedSeat => {
                        if count_visible_occupied(&cur_layout, height, width, i, j) >= 5 {
                            change_flag = true;
                            PositionState::EmptySeat
                        } else {
                            PositionState::OccupiedSeat
                        }
                    }
                };
                new_layout[i][j] = new_state;
            }
        }
        cur_layout = new_layout.clone();
        if !change_flag {
            break;
        }
    }

    let mut counter = 0;
    for row in cur_layout {
        for state in row {
            if state == PositionState::OccupiedSeat {
                counter += 1;
            }
        }
    }
    println!("There are {} occupied seats", counter);
}
