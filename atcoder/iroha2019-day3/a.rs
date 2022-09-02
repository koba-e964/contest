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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let a: i64 = get();
    let b: i64 = get();
    let c: i64 = get();
    let d: i64 = get();
    let e: i64 = get();
    let f: i64 = get();
    let g: i64 = get();
    let h: i64 = get();
    let i: i64 = get();
    let j: u32 = get();
    let k: i64 = get();
    let l: i64 = get();
    let m: i64 = get();
    let n: i64 = get();
    let o: i64 = get();
    let p: i64 = get();
    let q: i64 = get();
    let r: i64 = get();
    let s: i64 = get();
    let t: i64 = get();
    let u: i64 = get();
    let v: i64 = get();
    let w: i64 = get();
    let x: i64 = get();
    let y: i64 = get();
    let z: i64 = get();
    println!("{}", a - b);
    println!("{}", c + d);
    println!("{}", max(0, f + 1 - e));
    println!("{}", (g + h + i) / 3 + 1);
    let mut fav = "{".to_owned();
    let dag: Vec<_> = "dagabaji".chars().collect();
    for bits in 0i32..1 << 8 {
        if bits.count_ones() != j { continue; }
        let mut s = "".to_owned();
        for i in 0..8 {
            if (bits & 1 << i) != 0 {
                s.push(dag[i]);
            }
        }
        if fav > s {
            fav = s;
        }
    }
    println!("{}", fav);
    let mut val1 = 0;
    for i in 0..3599 {
        if i % 59 == k && i % 61 == l {
            val1 = i;
            break;
        }
    }
    val1 += 3599 * (m - 1);
    let perf = [6, 28, 496, 8128, 8191 * 4096];
    let mut val2 = 0;
    for &p in &perf {
        if (p - val1).abs() >= n {
            val2 = p;
            break;
        }
    }
    println!("{}", min(val1, val2));
    println!("{}", max(val1, val2));
    println!("{}", (o + p + q) * (r + s + t) * (u + v + w) * (x + y + z) % 9973);
}
