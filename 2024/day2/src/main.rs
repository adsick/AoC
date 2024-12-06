use aoc::read_input_rows;

fn is_safe(row: &[u8]) -> bool {
    let mut pd = row[1] as i16 - row[0] as i16;

    for w in row.windows(2) {
        let d = w[1] as i16 - w[0] as i16;

        if d * pd < 0 {
            return false;
        }

        let ad = d.abs();
        if ad == 0 || ad > 3 {
            return false;
        }
        pd = d;
    }
    true
}

fn is_safe_dampened(row: &[u8]) -> bool {
    for i in 0..row.len() {
        let mut row = row.to_vec();
        row.remove(i);
        if is_safe(&row) {
            return true;
        }
    }
    false
}

fn main() {
    let ans = read_input_rows::<u8>().filter(|r|is_safe_dampened(&r)).count();
    println!("{ans}")
}
