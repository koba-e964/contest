use std::cmp::*;
use std::io::Read;

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
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: i64 = get();
        let mut ab = vec![];
        let mut sa = 0;
        let mut sb = 0;
        let mut rng = vec![];
        let mut s = 0;
        let mut t = 0;
        for _ in 0..n {
            let a: i64 = get();
            let b: i64 = get();
            ab.push((a, b));
            sa += a;
            sb += b;
            let z = max(m - b, 0);
            let w = min(m, a);
            rng.push((z, w));
            s += z;
            t += w;
        }
        let goal = (sa + sb - m * n as i64) / 2;
        let mut rem = min(t, max(s, sa - goal));
        println!("{}", ((sa - rem) - (sb + rem - m * n as i64)).abs());
        rem -= s;
        for i in 0..n {
            let (z, w) = rng[i];
            let rx = z + min(w - z, rem);
            rem -= rx - z;
            println!("{} {}", rx, m - rx);
        }
    }
}
