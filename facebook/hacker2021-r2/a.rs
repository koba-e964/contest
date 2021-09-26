#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn calc(s: &[(i64, bool)], t: &[i64]) -> (i64, Vec<(i64, bool)>) {
    let m = s.len();
    let mut ans = m;
    let mut pa1 = HashMap::new();
    let mut pa2 = HashMap::new();
    let mut qa = HashMap::new();
    for i in 0..m {
        let (v, b) = s[i];
        if b {
            *pa2.entry(v).or_insert(0) += 1;
        } else {
            *pa1.entry(v).or_insert(0) += 1;
        }
        *qa.entry(t[i]).or_insert(0) += 1;
    }
    let mut ret = vec![];
    let mut unused = 0;
    let mut keys: Vec<_> = qa.keys().cloned().collect();
    keys.extend(pa1.keys().cloned());
    keys.extend(pa2.keys().cloned());
    keys.sort_unstable(); keys.dedup();
    for k in keys {
        let mut v = *qa.get(&k).unwrap_or(&0);
        let mut a1 = *pa1.get(&k).unwrap_or(&0);
        let a2 = *pa2.get(&k).unwrap_or(&0);
        ans -= min(v, a1 + a2);
        let r = min(a2, v);
        for _ in 0..r {
            ret.push((k, true));
        }
        v -= r;
        let r = min(a1, v);
        for _ in 0..r {
            ret.push((k, false));
        }
        v -= r;
        a1 -= r;
        unused += a1;
        for _ in 0..v {
            ret.push((k, true));
        }
    }
    ans -= unused;
    (ans as i64, ret)
}

fn solve() {
    let t: usize = get();
    for case_nr in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut s: Vec<(i64, bool)> = (0..m).map(|_| (get(), false)).collect();
        let mut p: Vec<Vec<i64>> = (0..n).map(|_| {
            (0..m).map(|_| get()).collect()
        }).collect();
        s.sort_unstable();
        for i in 0..n {
            p[i].sort_unstable();
        }
        let mut tot = 0;
        for i in 0..n {
            let (val, ns) = calc(&s, &p[i]);
            tot += val;
            s = ns;
        }
        println!("Case #{}: {}", case_nr + 1, tot);
    }
}
