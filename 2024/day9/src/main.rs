const INPUT: &str = include_str!("../input.txt");

// struct FsIter<I>(I, (u8, u8), usize);

// impl<I> FsIter<I> {
//     fn new(iter: I) -> Self {
//         FsIter(iter, (0, 0), 0)
//     }
// }

// impl<I: Iterator<>> Iterator for FsIter<I> {
//     type Item = u8;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.1.0 > 0 {

//         }
//     }
// }

fn main() {
    let mut b: Vec<u8> = INPUT
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    // padding
    if b.len() % 2 == 1 {
        b.push(0);
    }

    let mut blocks = b.chunks_exact(2).enumerate();

    let mut checksum: u64 = 0;

    let mut i: u64 = 0;
    // let mut j =

    let mut rem: u8 = 0;
    let mut rembi: u64 = 0;

    // figure out stop criteria!
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

    println!("ans: {checksum}");
}
