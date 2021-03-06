#![feature(destructuring_assignment)]
use std::{fs::File, io, io::prelude::*};

// todo: good way to rewrite this is to have a general solution for n dimensions

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day17/input.txt")?.read_to_string(&mut buffer)?;
    let mut conway_cube: Vec<Vec<Vec<Vec<char>>>> = vec![vec![buffer
        .split("\n")
        .map(|s| s.trim().chars().collect())
        .collect()]];

    for _ in 0..6 {
        conway_cube = pad_with_neighbors(&conway_cube);
        let mut new_cube = conway_cube.clone();
        for c in 0..conway_cube.len() {
            for z in 0..conway_cube[c].len() {
                for y in 0..conway_cube[c][z].len() {
                    for x in 0..conway_cube[c][z][y].len() {
                        let res = count_active_neighbors(&conway_cube, c, z, y, x);
                        if conway_cube[c][z][y][x] == '#' {
                            if res != 2 && res != 3 {
                                new_cube[c][z][y][x] = '.';
                            }
                        } else if conway_cube[c][z][y][x] == '.' {
                            if res == 3 {
                                new_cube[c][z][y][x] = '#';
                            }
                        }
                    }
                }
            }
        }
        conway_cube = new_cube;
    }

    let mut t = 0;
    for c in 0..conway_cube.len() {
        for z in 0..conway_cube[c].len() {
            for y in 0..conway_cube[c][z].len() {
                for x in 0..conway_cube[c][z][y].len() {
                    if conway_cube[c][z][y][x] == '#' {
                        t += 1;
                    }
                }
            }
        }
    }

    println!("{}", t);
    Ok(())
}

fn pad_with_neighbors(v: &Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut padded_c =
        vec![vec![vec![vec!['.'; v[0][0][0].len() + 2]; v[0][0].len() + 2]; v[0].len() + 2]; 2];
    for c in 0..v.len() {
        let mut padded_z = vec![vec![vec!['.'; v[c][0][0].len() + 2]; v[c][0].len() + 2]; 2];
        for z in 0..v[c].len() {
            let mut padded_y = vec![vec!['.'; v[c][z][0].len() + 2]; 2];
            for y in 0..v[c][z].len() {
                let mut padded_x = vec!['.', '.'];
                for x in 0..v[c][z][y].len() {
                    padded_x.insert(x + 1, v[c][z][y][x]);
                }
                padded_y.insert(y + 1, padded_x);
            }
            padded_z.insert(z + 1, padded_y);
        }

        padded_c.insert(c + 1, padded_z);
    }

    padded_c
}

fn count_active_neighbors(
    v: &Vec<Vec<Vec<Vec<char>>>>,
    c: usize,
    z: usize,
    y: usize,
    x: usize,
) -> i32 {
    let mut active_neighbors = 0;
    for mov_c in 0..3 {
        for mov_z in 0..3 {
            for mov_y in 0..3 {
                for mov_x in 0..3 {
                    if mov_z == 1 && mov_y == 1 && mov_x == 1 && mov_c == 1 {
                        continue;
                    }

                    let neighbor_c = c as i32 + mov_c - 1;
                    let neighbor_z = z as i32 + mov_z - 1;
                    let neighbor_y = y as i32 + mov_y - 1;
                    let neighbor_x = x as i32 + mov_x - 1;

                    let exists = neighbor_x >= 0
                        && neighbor_y >= 0
                        && neighbor_z >= 0
                        && neighbor_c >= 0
                        && neighbor_c < v.len() as i32
                        && neighbor_z < v[c].len() as i32
                        && neighbor_y < v[c][z].len() as i32
                        && neighbor_x < v[c][z][y].len() as i32;

                    if !exists {
                        continue;
                    } else {
                        if v[neighbor_c as usize][neighbor_z as usize][neighbor_y as usize]
                            [neighbor_x as usize]
                            == '#'
                        {
                            active_neighbors += 1;
                        }
                    }
                }
            }
        }
    }
    active_neighbors
}
