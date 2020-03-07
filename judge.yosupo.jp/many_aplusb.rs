use std::io::{Write, BufWriter};

fn read_nonneg_i64<I: Iterator<Item = u8>>(iter: &mut I) -> i64 {
    // non-neg only
    let mut v: i64 = 0;
    for c in iter.skip_while(|&c|c <= 0x20)
        .take_while(|&c|c > 0x20) {
            v = 10 * v + c as i64 - b'0' as i64;
        }
    v
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let stdin = std::io::stdin();
    let bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
    let mut bytes = bytes.map(|x| x.unwrap());
    let n = read_nonneg_i64(&mut bytes);
    for _ in 0..n {
        let a = read_nonneg_i64(&mut bytes);
        let b = read_nonneg_i64(&mut bytes);
        puts!("{}\n", a + b);
    }
}
