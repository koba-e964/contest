use std::io::{Read, Write, BufWriter};

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn calc(a: &[Vec<u32>]) -> u32 {
    let n = a.len();
    let mut ans = 0;
    for i in 0..n / 2 {
        if i % 2 == 0 {
            for j in 0..i / 2 + 1 {
                ans ^= a[2 * i + 1][4 * j];
            }
            for j in 0..i / 2 {
                ans ^= a[4 * j + 2][2 * i + 1];
            }
        } else {
            for j in 0..i / 2 + 1 {
                ans ^= a[2 * i + 1][4 * j + 2];
            }
            for j in 0..i / 2 + 1 {
                ans ^= a[4 * j][2 * i + 1];
            }
        }
    }
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let mut a = vec![vec![0u32; n]; n];
        for i in 0..n {
            for j in 0..n {
                a[i][j] = get();
            }
        }
        let mut v = calc(&a);
        for i in 0..n {
            a[i].reverse();
        }
        v ^= calc(&a);
        puts!("{}\n", v);
    }
}
