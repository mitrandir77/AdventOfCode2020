#![feature(generators, generator_trait)]

use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq)]
enum PositionState {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn adjacent_seats(height: usize, width: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let (i, j, width, height) = (i as isize, j as isize, width as isize, height as isize);
    for ii in i - 1..=i + 1 {
        for jj in j - 1..=j + 1 {
            if jj < width && ii < height && jj >= 0 && ii >= 0 && (ii != i || jj != j) {
                res.push((ii as usize, jj as usize));
            }
        }
    }
    res
}

fn count_adjacent(
    seating_layout: &[Vec<PositionState>],
    height: usize,
    width: usize,
    i: usize,
    j: usize,
    count_state: PositionState,
) -> usize {
    let mut counter = 0;
    for (ii, jj) in adjacent_seats(height, width, i, j) {
        if seating_layout[ii][jj] == count_state {
            counter += 1;
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
        println!("");
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
                        if count_adjacent(
                            &cur_layout,
                            height,
                            width,
                            i,
                            j,
                            PositionState::OccupiedSeat,
                        ) == 0
                        {
                            change_flag = true;
                            PositionState::OccupiedSeat
                        } else {
                            PositionState::EmptySeat
                        }
                    }
                    PositionState::OccupiedSeat => {
                        if count_adjacent(
                            &cur_layout,
                            height,
                            width,
                            i,
                            j,
                            PositionState::OccupiedSeat,
                        ) >= 4
                        {
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
