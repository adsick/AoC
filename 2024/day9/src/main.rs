use std::fmt::{Debug, Display};

const INPUT: &str = include_str!("../input.txt");

fn solve1(blocks: &[u8]) -> u64 {
    let mut blocks = blocks.chunks_exact(2).enumerate();

    let mut checksum: u64 = 0;

    let mut i: u64 = 0;

    let mut rem: u8 = 0;
    let mut rembi: u64 = 0;

    'o: loop {
        let Some((bi, f)) = blocks.next() else {
            break;
        };

        for _ in 0..f[0] {
            checksum += i * bi as u64;
            i += 1;
        }

        for _ in 0..f[1] {
            if rem == 0 {
                let Some((bi, f)) = blocks.next_back() else {
                    break 'o;
                };

                rem = f[0];
                rembi = bi as u64;
            }

            checksum += i * rembi;
            i += 1;
            rem -= 1;
        }
    }

    while rem > 0 {
        checksum += i * rembi;
        i += 1;
        rem -= 1;
    }

    println!("i: {i}");
    println!("rem: {rem}");
    println!("rembi: {rembi}");

    checksum
}

#[derive(Clone, Copy, Debug)]
struct Block {
    ind: usize,
    files: u16,
    space: u16,
    moved: bool,
}

impl Block {
    fn new(ind: usize, b: &[u8]) -> Self {
        let res = Self {
            ind,
            files: b[0] as u16,
            space: b[1] as u16,
            moved: false,
        };

        if res.files == 0 {
            println!("found an empty file block");
        }

        res
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.files {
            write!(f, "{} ", self.ind)?;
        }
        for _ in 0..self.space {
            write!(f, ". ")?
        }

        Ok(())
    }
}

fn solve2(blocks: &[u8]) -> u64 {
    let mut total: u64 = 0;

    let mut blocks: Vec<Block> = blocks
        .chunks_exact(2)
        .enumerate()
        .map(|(i, b)| {
            let b = Block::new(i, b);
            total += b.files as u64 + b.space as u64;
            b
        })
        .collect();

    #[cfg(feature = "stdout")]
    for (i, b) in blocks.iter().enumerate() {
        println!("{i:<5}: {b:?}")
    }

    println!("total: {total}");

    #[cfg(feature = "huge")]
    println!("{:-<90}", "");

    let mut i = blocks.len() - 1;

    // initially I iterated using a for loop through (0..blocks.len()).rev()
    // but because I move blocks using insert, elements shift and when that happens I skipped elements
    while i > 0 {
        #[cfg(feature = "stdout")]
        println!("{i}");

        let mut bi = blocks[i];

        if bi.moved {
            i -= 1;
            continue;
        }

        #[cfg(feature = "huge")]
        {
            let mut res1 = String::new();
            let mut res2 = String::new();
            for b in blocks.iter() {
                res1 += &b.to_string();
                for _ in 0..b.files + b.space {
                    res2 += if b.ind == bi.ind { "| " } else { "  " }
                }
            }
            println!("{res1}");
            println!("{res2}");
        }

        // find a blocks that has enough space to fit the currently observed (blocks[i])
        let mut shift = false;
        for j in 0..i {
            if blocks[j].space >= bi.files {
                let bj = blocks[j];

                #[cfg(feature = "huge")]
                {
                    print!("bi ({i}): {bi:?} -> ");
                    println!("bj ({j}): {bj:?}");
                }

                #[cfg(feature = "stdout")]
                println!(
                    "{j:<05} <- {i:<05} i(#{: <4}, #{: <4}), f({}, {}), s({:<2}, {:<5})",
                    bj.ind, bi.ind, bj.files, bi.files, bj.space, bi.space
                );

                let bi_occupied = bi.space + bi.files;

                bi.space = bj.space - bi.files;

                bi.space = bj.space - bi.files;
                bi.moved = true;
                blocks[j].space = 0;

                blocks.remove(i);
                blocks.insert(j + 1, bi);

                blocks[i].space += bi_occupied;

                shift = true;

                #[cfg(feature = "huge")]
                {
                    let mut res = String::new();
                    for b in blocks.iter() {
                        res += &b.to_string();
                        for _ in 0..b.files + b.space {
                            if b.ind == blocks[j + 1].ind {
                                print!("| ")
                            } else {
                                print!("  ");
                            }
                        }
                    }

                    println!();
                    println!("{res}");
                }
                break;
            }
        }

        if !shift {
            i -= 1;
        }

        #[cfg(feature = "huge")]
        println!("{:-<90}", "");
    }

    let mut checksum = 0;

    let mut i = 0;
    for b in blocks {
        #[cfg(feature = "stdout")]
        print!("{b}");

        for _ in 0..b.files {
            checksum += b.ind as u64 * i;
            i += 1;
        }
        i += b.space as u64;
    }
    println!("total: {i}");

    checksum
}

fn main() {
    let mut blocks: Vec<u8> = INPUT
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    // padding for `chunks_exact` (I'm a chill guy)
    if blocks.len() % 2 == 1 {
        println!("padded");
        blocks.push(0);
    }

    println!("ans1: {}", solve1(&blocks));
    println!("ans2: {}", solve2(&blocks));
    // println!("0 0 9 9 2 1 1 1 7 7 7 . 4 4 . 3 3 3 . . . . 5 5 5 5 . 6 6 6 6 . . . . . 8 8 8 8 . .")
}
