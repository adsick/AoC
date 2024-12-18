const INPUT: &str = include_str!("../input.txt");
const SIZE: usize = 71; // 70

use std::collections::VecDeque;

use aoc::Pos;
use colored::{Color, Colorize};

type Mat = Vec<Vec<u32>>;

// c = current distance
fn bfs(p: Pos<usize>, c: u32, obst: &Mat, d: &mut Mat, queue: &mut VecDeque<(Pos<usize>, u32)>) {
    let pd = &mut d[p.1][p.0];

    if c > *pd {
        return;
    }

    *pd = c;

    for di in -1..=1isize {
        for dj in -1..=1isize {
            if di * dj != 0 || (di, dj) == (0, 0) {
                continue;
            }

            let pi = p.1 as isize + di;
            let pj = p.0 as isize + dj;

            // println!("{pi}, {pj}");

            if pi >= 0 && pj >= 0 {
                let pi = pi as usize;
                let pj = pj as usize;

                if pi < SIZE && pj < SIZE && obst[pi][pj] == 0 && d[pi][pj] > c + 1 {
                    queue.push_back((Pos(pj, pi), c + 1))
                }
            }
        }
    }
}

fn main() {
    let mut obstacles = vec![vec![0; SIZE]; SIZE];

    for l in INPUT.lines().take(1024) {
        let p: Pos<usize> = l.parse().unwrap();
        // println!("OBSTACLE: {p}");
        obstacles[p.1][p.0] = 1;
    }

    'o: for (i, l) in INPUT.lines().skip(1024).enumerate() {
        let op: Pos<usize> = l.parse().unwrap();
        println!("OBSTACLE{}: {op}", i + 1024);
        obstacles[op.1][op.0] = 1;

        let mut da = vec![vec![u32::MAX; SIZE]; SIZE];
        let mut db = vec![vec![u32::MAX; SIZE]; SIZE];

        let mut queue_a = VecDeque::new();
        let mut queue_b = VecDeque::new();
        queue_a.push_back((Pos(0, 0), 0));
        queue_b.push_back((Pos(SIZE - 1, SIZE - 1), 1));

        let mut i = 0;
        let ans = loop {
            if queue_a.len() < queue_b.len() {
                let Some((p, c)) = queue_a.pop_front() else {
                    break 'o;
                };

                if db[p.1][p.0] < u32::MAX {
                    break; //c + db[p.1][p.0];
                }
                bfs(p, c, &obstacles, &mut da, &mut queue_a);
            } else {
                let Some((p, c)) = queue_b.pop_front() else {
                    break 'o;
                };

                if da[p.1][p.0] < u32::MAX {
                    break; //c + da[p.1][p.0];
                }
                bfs(p, c, &obstacles, &mut db, &mut queue_b);
            }

            if i % 100000 == 0 {
                // println!("qa: {}, qb: {}", queue_a.len(), queue_b.len());
                queue_a.retain(|(p, _c)| da[p.1][p.0] == u32::MAX);
                queue_b.retain(|(p, _c)| db[p.1][p.0] == u32::MAX);
                // println!("qa: {}, qb: {}", queue_a.len(), queue_b.len());
                for i in 0..SIZE {
                    for j in 0..SIZE {
                        let o = obstacles[i][j];
                        let ca = da[i][j];
                        let cb = db[i][j];
                        if o > 0 {
                            if Pos(j, i) == op {
                                print!("{}", "# ".color(Color::Black).on_color(Color::BrightYellow))
                            } else {
                                print!("# ");
                            }
                            continue;
                        }
                        if ca < u32::MAX && cb < u32::MAX {
                            print!("X ")
                        } else if ca < u32::MAX {
                            print!("A ")
                        } else if cb < u32::MAX {
                            print!("B ")
                        } else {
                            print!("  ")
                        }
                    }
                    println!();
                }
                println!();
            }
            i += 1;
        };
    }
    // let ans = d[SIZE - 1][SIZE - 1];

    // println!("ans1: {ans}")
}
