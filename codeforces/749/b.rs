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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut con = vec![];
        let mut f = vec![0; n];
        for _ in 0..m {
            let a = get::<usize>() - 1;
            let b = get::<usize>() - 1;
            let c = get::<usize>() - 1;
            con.push((b, (a, c)));
            f[b] += 1;
        }
        let x = (0..n).position(|i| f[i] == 0).unwrap();
        for i in 0..n {
            if x != i {
                puts!("{} {}\n", x + 1, i + 1);
            }
        }
    }
}
