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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input!(n: i64, k: usize);
    const W: usize = 1_000_001;
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut hm = HashMap::new();
    let mut tbl = vec![0; k];
    for i in 0..k {
        tbl[i] = n - i as i64;
    }
    for p in 2..W {
        if !pr[p] { continue; }
        let pp = p as i64;
        let r = (n % pp) as usize;
        if k > r {
            for i in 0..(k - r - 1) / p + 1 {
                let idx = p * i + r;
                let mut v = tbl[idx];
                while v % pp == 0 {
                    v /= pp;
                    *hm.entry(pp).or_insert(0) += 1;
                }
                tbl[idx] = v;
            }
        }
        let mut v = k as i64;
        let mut sub = 0;
        while v > 0 {
            v /= pp;
            sub += v;
        }
        if sub > 0 {
            *hm.get_mut(&pp).unwrap() -= sub;
        }
    }
    for i in 0..k {
        if tbl[i] > 1 {
            *hm.entry(tbl[i]).or_insert(0) += 1;
        }
    }
    let mut ans = 1;
    for (_, v) in hm {
        ans = ans * (v + 1) % 998_244_353;
    }
    println!("{}", ans);
}
