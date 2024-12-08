use std::{collections::BTreeSet, thread::sleep, time::Duration};

const INPUT: &str = include_str!("../input.txt");

// directions:
// 0 UP
// 1 RIGHT
// 2 DOWN
// 3 LEFT

type Mat = Vec<Vec<u8>>;
type Guard = (usize, usize, u8);

// map:
// 0 nothing
// 1 wall
// 2-30 visited directions:
// 2 UP
// 4 RIGHT
// 8 DOWN
// 16 LEFT
// 32 rock

const MASK: u8 = 0b11110;

fn out_of_bounds(map: &Mat, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || i >= map.len() as isize || j >= map.len() as isize
}

fn wall(map: &Mat, i: usize, j: usize) -> bool {
    map[i as usize][j as usize] == 1 || map[i as usize][j as usize] == 32
}

// returns true if location is newly visited
fn visit(map: &mut Mat, guard: &Guard) -> bool {
    let cell = &mut map[guard.0 as usize][guard.1 as usize];

    let first_visit = *cell == 0;

    *cell |= 2 << guard.2;
    first_visit
}

// returns true if loop is detected
fn visit_loops(map: &mut Mat, guard: &Guard) -> bool {
    let cell = &mut map[guard.0 as usize][guard.1 as usize];

    let loop_detected = *cell & 2 << guard.2 > 0;
    // print!("cell {cell} |= 2 << {} = ", guard.2);
    *cell |= 2 << guard.2;

    // println!(
    //     "{cell}. {}",
    //     if loop_detected { "loop detected" } else { "" }
    // );
    loop_detected
}

fn turn(guard: &mut Guard) {
    guard.2 += 1;
    guard.2 %= 4;
}

fn step(map: &Mat, guard: &Guard) -> Option<(usize, usize)> {
    let v: (isize, isize) = match guard.2 {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => unreachable!(),
    };

    let i = guard.0 as isize + v.0;
    let j = guard.1 as isize + v.1;

    if out_of_bounds(map, i, j) {
        return None;
    }

    Some((i as usize, j as usize))
}

fn mov(map: &Mat, guard: &mut Guard) -> bool {
    let Some((i, j)) = step(map, guard) else {
        return false;
    };

    if wall(map, i, j) {
        turn(guard);
        return mov(map, guard);
    }

    guard.0 = i;
    guard.1 = j;

    true
}

fn print_map(map: &Mat, guard: &Guard) {
    for (i, r) in map.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            let ch = match c {
                0 => '.',
                1 => '#',
                _ if (guard.0, guard.1) == (i, j) => match guard.2 {
                    0 => '^',
                    1 => '>',
                    2 => 'v',
                    3 => '<',
                    _ => unreachable!(),
                },
                32 => 'O',
                b if b & MASK > 0 => 'X',
                _ => unreachable!(),
            };
            print!("{ch}")
        }
        println!();
    }
}

fn main() {
    let mut guard: Guard = (0, 0, 0);

    let mut map: Mat = INPUT
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => 0,
                    '#' => 1,
                    '^' => {
                        (guard.0, guard.1) = (i, j);
                        0
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut x = 0;

    let start_guard = guard;
    let start_map = map.clone();
    let mut rock_candidates = BTreeSet::new();

    // for whatever reason display of the guard broken
    loop {
        if visit(&mut map, &guard) {
            x += 1;
            rock_candidates.insert((guard.0, guard.1));
            println!("{x}");
        } else {
            println!()
        }

        if !mov(&mut map, &mut guard) {
            // out of bounds
            break;
        }
    }
    print_map(&map, &guard);

    println!("ans: {x}");
    println!("Part 2");

    let mut unique_rock_positions = BTreeSet::new();

    for (n, (i, j)) in rock_candidates.into_iter().enumerate() {
        let mut map = start_map.clone();
        let mut guard = start_guard;

        // set the rock
        map[i][j] = 32;

        loop {
            if visit_loops(&mut map, &guard) {
                println!("loop found in ({i}, {j}), candidate #{n}");
                print_map(&map, &guard);
                println!();

                unique_rock_positions.insert((i, j));
                break;
            }
            if !mov(&map, &mut guard) {
                println!("out of bounds");
                break;
            }
        }
    }

    println!("ans2: {}", unique_rock_positions.len());
}
