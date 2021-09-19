use std::collections::*;
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn swap(s: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut t = HashSet::new();
    for &(x, y) in s {
        t.insert((y, x));
    }
    t
}

fn neg(s: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut t = HashSet::new();
    for &(x, y) in s {
        t.insert((-x, y));
    }
    t
}

fn neg_y(s: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut t = HashSet::new();
    for &(x, y) in s {
        t.insert((x, -y));
    }
    t
}

fn sing(s: &HashSet<(i64, i64)>, dx: i64, k: i64) -> i64 {
    assert!(dx > 0);
    let mut hm = HashMap::new();
    for &(x, y) in s {
        let rx = (x % dx + dx) % dx;
        hm.entry((rx, y)).or_insert(vec![]).push((x - rx) / dx);
    }
    // eprintln!("{:?}", hm);
    let mut tot = 0;
    for (_, v) in &hm {
        let mut v = v.clone();
        v.sort();
        let mut hi = v[0] + k - 1;
        tot += k;
        for i in 1..v.len() {
            let l = v[i];
            let r = v[i] + k - 1;
            if l <= hi {
                tot += r - hi;
            } else {
                tot += r - l + 1;
            }
            hi = r;
        }
    }
    tot
}

fn dual(s: &HashSet<(i64, i64)>, dx: i64, dy: i64, k: i64) -> i64 {
    assert!(dx > 0 && dy > 0);
    let mut hm = HashMap::new();
    for &(x, y) in s {
        let rx = (x % dx + dx) % dx;
        let q = (x - rx) / dx;
        hm.entry((rx, y - dy * q)).or_insert(vec![]).push(q);
    }
    // eprintln!("{:?}", hm);
    let mut tot = 0;
    for (_, v) in &hm {
        let mut v = v.clone();
        v.sort();
        let mut hi = v[0] + k - 1;
        tot += k;
        for i in 1..v.len() {
            let l = v[i];
            let r = v[i] + k - 1;
            if l <= hi {
                tot += r - hi;
            } else {
                tot += r - l + 1;
            }
            hi = r;
        }
    }
    tot
}

fn main() {
    input! {
        s: chars,
        k: i64,
    }
    let mut seen = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    seen.insert((0, 0));
    for i in 0..s.len() {
        match s[i] {
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y -= 1,
            'D' => y += 1,
            _ => panic!(),
        }
        seen.insert((x, y));
    }
    if x == 0 && y == 0 {
        println!("{}", seen.len());
        return;
    }
    if y == 0 {
        if x < 0 {
            seen = neg(&seen);
            x = -x;
        }
        println!("{}", sing(&seen, x, k));
        return;
    }
    if x == 0 {
        seen = swap(&seen);
        if y < 0 {
            seen = neg(&seen);
            y = -y;
        }
        println!("{}", sing(&seen, y, k));
        return;
    }
    if x < 0 {
        seen = neg(&seen);
        x = -x;
    }
    if y < 0 {
        seen = neg_y(&seen);
        y = -y;
    }
    println!("{}", dual(&seen, x, y, k));
}
