use std::{collections::{HashMap, HashSet}, thread::sleep, time::Duration};

const INPUT: &str = include_str!("../input.txt");
const OUTPUT: &str = include_str!("../small-output.txt");

type Mat = Vec<Vec<char>>;
type Antennas = HashMap<char, HashSet<Antenna>>;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Antenna {
    i: isize,
    j: isize,
}

fn main() {
    let s = INPUT.lines().next().unwrap().len();
    let mut map = vec![vec![0u8; s]; s];

    let mut antennas = Antennas::new();

    for (i, r) in INPUT.lines().enumerate() {
        for (j, c) in r.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let a = Antenna {
                i: i as isize,
                j: j as isize,
            };
            antennas.entry(c).or_default().insert(a);
        }
    }

    let mut x = 0;

    for (c, anns) in antennas {
        let mut processed = HashSet::new();

        println!("ans: {x}");

        for (n1, a1) in anns.iter().enumerate() {
            println!("start from {n1} with coords {a1:?}");
            for (n2, a2) in anns.iter().enumerate() {
                if n1 == n2 {
                    println!("skip the same ({n1:?})");
                    continue;
                }
                let p1 = processed.insert((n1, n2));
                let p2 = processed.insert((n2, n1));
                // wrong(?)
                if !p1 || !p2 {
                    println!("skip processed ({n1}, {n2})", );
                    continue;
                }
                println!("processing ({n1}, {n2}) with coords {a1:?}, {a2:?}");


                let di = a2.i - a1.i;
                let dj = a2.j - a1.j;

                // let i1 = a1.i - di;
                // let i2 = a2.i + di;

                // let j1 = a1.j - dj;
                // let j2 = a2.j + dj;


                let mut k = 1;
                for _ in 0..2 {
                    let mut i = a1.i;
                    let mut j = a1.j;

                    while i >= 0 && i < s as isize && j >= 0 && j < s as isize {
                        let p = &mut map[i as usize][j as usize];
    
                        if *p == 0 {
                            *p = c as _;
                            x += 1;
                        }
    
                        i += k * di;
                        j += k * dj;
                    }
                    k *= -1;
                }


                // println!("candidates: ({i1}, {j1}), ({i2}, {j2})");

                // if i1 >= 0 && i1 < s as isize {
                //     if j1 >= 0 && j1 < s as isize {
                //         let p = &mut map[i1 as usize][j1 as usize];

                //         if *p == 0 {
                //             *p = c as _;
                //             x += 1;
                //         }
                //     }
                // }

                // if i2 >= 0 && i2 < s as isize {
                //     if j2 >= 0 && j2 < s as isize {
                //         let p = &mut map[i2 as usize][j2 as usize];

                //         if *p == 0 {
                //             *p = c as _;
                //             x += 1;
                //         }
                //     }
                // }
            }

            for r in map.iter() {
                for c in r {
                    print!("{:<02} ", *c as char);
                }
                println!();
            }
            // sleep(Duration::from_millis(200));
        }
    }

    println!("ans: {x}");

    for r in map.iter() {
        for c in r {
            print!("{:<02} ", *c as char);
        }
        println!();
    }

    println!("---------------------------------");

    // for (i, r) in OUTPUT.lines().enumerate() {
    //     for (j, c) in r.chars().enumerate() {

    //         let e = c == '#';

    //         let a = map[i][j] > 0;

    //         if a != e {
    //             if a {
    //                 print!("E ")
    //             } else {
    //                 print!("e ")
    //             }
    //         } else if e {
    //             print!("# ")
    //         } else {
    //             print!("  ")
    //         }
    //     }
    //     println!();
    // }
}
