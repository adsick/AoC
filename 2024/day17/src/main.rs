use std::{str::FromStr, thread};

const INPUT: &str = include_str!("../input.txt");

#[derive(Default, Debug, Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    p: Vec<u8>,
    i: usize,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ls = s.lines();

        let mut s = State::default();

        s.a = ls.next().ok_or(())?[12..].parse().unwrap();
        s.b = ls.next().ok_or(())?[12..].parse().unwrap();
        s.c = ls.next().ok_or(())?[12..].parse().unwrap();
        ls.next();

        s.p = ls.next().ok_or(())?[9..]
            .split(',')
            .map(|b| b.parse().unwrap())
            .collect();

        Ok(s)
    }
}

impl State {
    fn combo(&self, o: u8) -> u64 {
        match o {
            l @ 0..4 => l as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, o: u8) {
        self.a = self.a / (1 << self.combo(o))
    }

    fn bxl(&mut self, o: u8) {
        self.b = self.b ^ o as u64
    }

    fn bst(&mut self, o: u8) {
        self.b = self.combo(o) % 8
    }

    fn jnz(&mut self, o: u8) -> bool {
        if self.a != 0 {
            self.i = o as usize;
            false
        } else {
            true
        }
    }

    fn bxc(&mut self, _o: u8) {
        self.b = self.b ^ self.c
    }

    fn out(&self, o: u8) -> u8 {
        (self.combo(o) % 8 as u64) as u8
    }

    fn bdv(&mut self, o: u8) {
        self.b = self.a / (1 << self.combo(o))
    }

    fn cdv(&mut self, o: u8) {
        self.c = self.a / (1 << self.combo(o))
    }

    fn run(&mut self) -> Option<Option<u8>> {
        let s = self;
        let mut out = None;
        let Some([c, o]) = s.p.get(s.i..s.i + 2) else {
            return None;
        };

        let mut increment = true;
        match c {
            0 => s.adv(*o),
            1 => s.bxl(*o),
            2 => s.bst(*o),
            3 => {
                increment = s.jnz(*o);
            }
            4 => s.bxc(*o),
            5 => out = Some(s.out(*o)),
            6 => s.bdv(*o),
            7 => s.cdv(*o),

            _ => unreachable!(),
        }

        if increment {
            s.i += 2;
        }

        Some(out)
    }

    fn runi(&mut self) -> Option<u64> {
        let s = self;
        let mut out = None;

        // the program should halt, so it's at the end

        // let mut i = s.p.len() - 2;
        
        //if self.a != 0

        // the last instruction is just a jump back if a != 0
        // so halt case means that a == 0

        s.a = 0;

        // previous operation is a /= 8 (`a /= 2^combo(3)`, combo(3) = 3)
        // so a = 0 if a < 8

        // next previous operations:
        // out(5) O = B % 8                 (comb(5) = B)

        // and this out(5) should also equal the current byte of the program!


        // bxc(*) B = B^C                   (argument is ignored)
        // bxl(6) B = B xor 6 = B xor 110
        // cdv(5) C = A / 2^B               (comb(5) = B)
        // bxl(5) B = B ^ 5
        // bst(4) B = A % 8

        // and here we came from previous iteration of the loop...

        // registers C and B should be 0 at the start 


        let Some([c, o]) = s.p.get(s.i..s.i + 2) else {
            return None;
        };

        let mut increment = true;
        match c {
            0 => s.adv(*o),
            1 => s.bxl(*o),
            2 => s.bst(*o),
            3 => {
                increment = s.jnz(*o);
            }
            4 => s.bxc(*o),
            5 => out = Some(s.out(*o)),
            6 => s.bdv(*o),
            7 => s.cdv(*o),

            _ => unreachable!(),
        }

        if increment {
            s.i += 2;
        }

        todo!()
    }
}

fn solve1(mut s: State) -> Vec<u8> {
    let mut out = vec![];
    while let Some(o) = s.run() {
        if let Some(o) = o {
            out.push(o)
        }
    }
    out
}

fn solve2(s: State, thrdn: u64, step: usize) -> u64 {
    let mut out = vec![];
    for a in (thrdn..u64::MAX).step_by(step) {
        if (a - thrdn) % (1 << 27) == 0 {
            println!("{thrdn} {a}");
        }
        let mut s = s.clone();
        s.a = a;
        out.clear();

        while let Some(o) = s.run() {
            if let Some(o) = o {
                if o != s.p[out.len()] {
                    // println!("{out:?} + {o}");
                    break;
                }
                out.push(o);
            }
        }

        if out == s.p {
            return a;
        }
    }

    0
}

fn main() {
    let s: State = INPUT.parse().unwrap();

    println!("{s:?}");
    let ans1 = solve1(s.clone())
        .into_iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("ans1: {ans1}");

    let n = 8;
    let mut handles = vec![];
    for i in 0..n {
        let s = s.clone();
        handles.push(thread::spawn(move || solve2(s, i, n as usize)));
    }

    let mut ans2 = 0;

    for h in handles {
        let a = h.join().unwrap();
        if a != 0 {
            ans2 = a;
        }
    }

    println!("ans2: {ans2}");
}
