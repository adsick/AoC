//    0 1 2 3 (4) w
// 0  a b c d \n
// 1  e f g h \n
// 2  a b c d \n
// 3  e f g h \n
// h

// (3, 0); (2, 0), (3, 1); (1, 0), (2, 1), (3, 2); (0, 0), ...; (0, 1)

type Mat = Vec<Vec<char>>;

fn count_xmas(mat: &Mat) -> u32 {
    let mut count = 0;
    let a = mat.len() as isize;
    let mut s = (0, 0);

    let mut feed = |s: &mut (u8, u8), c: &char| {
        // print!("{:?} + {c} = ", *s);
        s.0 = match (s.0, c) {
            (_, 'X') => 1,
            (1, 'M') => 2,
            (2, 'A') => 3,
            (3, 'S') => {
                count += 1;
                4
            }
            _ => 0,
        };
        s.1 = match (s.1, c) {
            (_, 'S') => 1,
            (1, 'A') => 2,
            (2, 'M') => 3,
            (3, 'X') => {
                count += 1;
                4
            }
            _ => 0,
        };
        // println!("{:?}", *s);
    };

    // rows
    for row in mat {
        row.iter().for_each(|c| feed(&mut s, c));
        s = (0, 0);
    }

    // columns
    let ir = 0..a as usize;

    for j in 0..a as usize {
        ir.clone().for_each(|i| feed(&mut s, &mat[i][j]));
        s = (0, 0);
    }

    // diags

    for i in -a..=a {
        let jr = 0..a - i.abs();
        for j in jr {
            let mut p = (j, j);

            p.0 += i.max(0);
            p.1 -= i.min(0);

            // println!("{p:?}");

            let c = mat[p.0 as usize][p.1 as usize];
            feed(&mut s, &c);
        }
        s = (0, 0);
    }

    for i in -a..=a {
        let jr = 0..a - i.abs();
        for j in jr {
            let mut p = (a - j - 1, j);

            p.0 -= i.max(0);
            p.1 -= i.min(0);

            // println!("{p:?}");

            let c = mat[p.0 as usize][p.1 as usize];
            feed(&mut s, &c);
        }
        s = (0, 0);
    }

    count
}

fn is_ms(c1: char, c2: char) -> bool {
    c1 == 'M' && c2 == 'S' || c1 == 'S' && c2 == 'M'
}

fn count_mas(m: &Mat) -> u32 {
    let mut count = 0;
    let a = m.len();

    for i in 0..a - 2 {
        for j in 0..a - 2 {
            if m[i + 1][j + 1] != 'A' {
                continue;
            }

            if is_ms(m[i][j], m[i + 2][j + 2]) && is_ms(m[i][j + 2], m[i + 2][j]) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");
    let mat: Mat = input
        .lines()
        .map(|s| s.chars().take(140).collect())
        .collect();

    // let s = mat.len() as isize;
    let a: isize = 3;
    println!("{a}");

    let c = count_mas(&mat);

    println!("ans: {c}");

    // for i in -a..=a {
    //     for j in 0..a - i.abs() {
    //         let mut p = (j, j);

    //         p.0 -= i.min(0);
    //         p.1 += i.max(0);

    //         print!("{p:?} ")
    //     }
    //     println!()
    // }

    // for i in -a..=a {
    //     for j in 0..a - i.abs() {
    //         let mut p = (a - j - 1, j);

    //         p.0 -= i.max(0);
    //         p.1 -= i.min(0);

    //         print!("{p:?} ")
    //     }
    //     println!()
    // }
}
