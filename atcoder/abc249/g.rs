use std::cmp::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, k: u64,
        ab: [(u64, u64); n],
    }
    let mut basis = vec![];
    let mut allow_zero = false;
    for (a, b) in ab {
        let mut v = a << 30 | b;
        for &b in &basis {
            v = min(v, v ^ b);
        }
        if v > 0 {
            basis.push(v);
        } else {
            allow_zero = true;
        }
    }
    basis.sort();
    let mut cur = k + 1;
    let mut ma = 0;
    let mut found = false;
    while cur > 0 {
        let rng = cur & cur.wrapping_neg();
        let st = cur - rng;
        // [st, st + rng)
        let mut val = st << 30;
        let mut idx = 0;
        for i in (0..basis.len()).rev() {
            if val < rng << 30 && basis[i] < rng << 30 {
                idx = i + 1;
                break;
            }
            val = min(val, val ^ basis[i]);
        }
        if idx == 0 {
            cur = st;
            continue;
        }
        found = true;
        val &= (1 << 30) - 1;
        let mut tmp = vec![];
        tmp.sort_unstable();
        for i in (0..idx).rev() {
            let mut x = basis[i] & ((1 << 30) - 1);
            for &b in &tmp {
                x = min(x, x ^ b);
            }
            if x > 0 {
                tmp.push(x);
            }
        }
        tmp.sort();
        for &v in tmp.iter().rev() {
            val = max(val, val ^ v);
        }
        ma = max(ma, val);
        cur = st;
    }
    if !found && !allow_zero {
        println!("-1");
        return;
    }
    println!("{}", ma);
}
