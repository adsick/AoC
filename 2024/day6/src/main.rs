use std::{thread::sleep, time::Duration};

const INPUT: &str = include_str!("../input.txt");

// directions:
// 0 UP
// 1 RIGHT
// 2 DOWN
// 3 LEFT

type Mat = Vec<Vec<u8>>;
type Guard = (isize, isize, u8);

// map:
// 0 nothing
// 1 wall
// 2 visited

fn out_of_bounds(map: &Mat, i: isize, j: isize) -> bool {
    i < 0 || j < 0 || i >= map.len() as isize || j >= map.len() as isize
}

fn wall(map: &Mat, i: isize, j: isize) -> bool {
    map[i as usize][j as usize] == 1
}

fn visit(map: &mut Mat, guard: &Guard) -> bool {
    let cell = &mut map[guard.0 as usize][guard.1 as usize];

    if *cell != 2 {
        *cell = 2;
        true
    } else {
        false
    }
}

fn turn(guard: &mut Guard) {
    guard.2 += 1;
    guard.2 %= 4;
}

fn step(map: &mut Mat, guard: &mut Guard) -> bool {
    let v: (isize, isize) = match guard.2 {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => unreachable!(),
    };

    let i = guard.0 + v.0;
    let j = guard.1 + v.1;

    if out_of_bounds(map, i, j) {
        return false;
    }

    if wall(map, i, j) {
        turn(guard);
        return true;
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
                _ if (guard.0, guard.1) == (i as isize, j as isize) => match guard.2 {
                    0 => '^',
                    1 => '>',
                    2 => 'v',
                    3 => '<',
                    _ => unreachable!(),
                },
                2 => 'X',
                _ => unreachable!(),
            };
            print!("{ch}")
        }
        println!();
    }
}

fn main() {
    let mut guard = (0, 0, 0);

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
                        (guard.0, guard.1) = (i as isize, j as isize);
                        0
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut x = 0;

    // visit(&mut map, &guard);

    // for whatever reason display of guard broken
    loop {
        if visit(&mut map, &guard) {
            x += 1;
            println!("{x}");
        } else {
            println!()
            // println!("already visited")
        }

        if !step(&mut map, &mut guard) {
            break;
        }

        // print_map(&map, &guard);
        // println!();
        sleep(Duration::from_millis(1));
        // clearscreen::clear().unwrap();
    }
    print_map(&map, &guard);


    println!("ans: {x}")
}
