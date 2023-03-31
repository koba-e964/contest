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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn calc(a: i64, b: i64, c: i64, d: i64) -> i64 {
    assert!(a < b);
    fn dist(src: i64, dst: i64) -> i64 {
        if src <= dst {
            dst - src
        } else {
            dst + 1
        }
    }
    if a > c || b <= d {
        let na = a > c;
        let nb = c > d || b > d;
        return if na {
            c + 1
        } else {
            c - a
        } + if nb {
            d + 1
        } else {
            d - b
        };
    }
    if c > d {
        return dist(a, c) + dist(b, d);
    }
    c + 1 + d + 1 + if a == 1 { 1 } else { 0 }
}

// https://yukicoder.me/problems/no/2213 (3)
// A < B としてよい。 A <= C < D < B のとき y を先に調整する必要があり、
// なおかつ A = 1 のとき (x, y) = (1, 1) を避けるために A = 2 にする必要があり面倒。
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let a: i64 = get();
        let b: i64 = get();
        let c: i64 = get();
        let d: i64 = get();
        println!("{}", if a < b { calc(a, b, c, d) } else { calc(b, a, d, c) });
    }
}
