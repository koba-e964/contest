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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// #{(i, j) | i in v1, j in v2, min in unique in [i, j]}
fn calc(v1: &[usize], v2: &[usize], acc: &[(i64, i32)]) -> i64 {
    let a = v1.len();
    let b = v2.len();
    let mut uniq_cnt = vec![0; b + 1];
    let mut uniq_sum = vec![0; b + 1];
    for i in (0..b).rev() {
        uniq_cnt[i] = uniq_cnt[i + 1] + if acc[v2[i]].1 == 1 {
            1
        } else {
            0
        };
        uniq_sum[i] = uniq_sum[i + 1] + if acc[v2[i]].1 == 1 {
            v2[i] as i64
        } else {
            0
        };
    }
    let mut idx_sum = vec![0; b + 1];
    for i in 0..b {
        idx_sum[i + 1] = idx_sum[i] + v2[i] as i64;
    }
    let mut ans = 0;
    let mut posgt = 0;
    let mut posge = 0;
    for i in (0..a).rev() {
        let (val, uniq) = acc[v1[i]];
        while posgt < b && acc[v2[posgt]].0 > val {
            posgt += 1;
        }
        while posge < b && acc[v2[posge]].0 >= val {
            posge += 1;
        }
        if uniq == 1 {
            ans += idx_sum[posgt] - (v1[i] as i64 - 1) * posgt as i64;
        }
        ans += uniq_sum[posge] - uniq_cnt[posge] * (v1[i] as i64 - 1);
    }
    ans
}

fn rec(l: usize, r: usize, a: &[i64], k: i64) -> i64 {
    if l >= r {
        return 0;
    }
    if l + 1 == r {
        return if 2 * a[l] == k { 1 } else { 0 };
    }
    let mid = (l + r) / 2;
    let mut tot = 0;
    tot += rec(l, mid, a, k);
    tot += rec(mid, r, a, k);
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    let mut acc = vec![(0, 0); r - l];
    acc[mid - l - 1] = (a[mid - 1], 1);
    acc[mid - l] = (a[mid], 1);
    for i in (l..mid - 1).rev() {
        if acc[i - l + 1].0 <= a[i] {
            acc[i - l] = acc[i - l + 1];
            if acc[i - l].0 == a[i] {
                acc[i - l].1 += 1;
            }
        } else {
            acc[i - l] = (a[i], 1);
        }
    }
    for i in mid + 1..r {
        if acc[i - l - 1].0 <= a[i] {
            acc[i - l] = acc[i - l - 1];
            if acc[i - l].0 == a[i] {
                acc[i - l].1 += 1;
            }
        } else {
            acc[i - l] = (a[i], 1);
        }
    }
    for i in l..r {
        if i < mid {
            left.entry(a[i]).or_insert(vec![]).push(i - l);
        } else {
            right.entry(a[i]).or_insert(vec![]).push(i - l);
        }
    }
    for (k1, v1) in left {
        let k2 = k - k1;
        if let Some(v2) = right.get(&k2) {
            tot += calc(&v1, v2, &acc);
        }
    }
    tot
}

// Tags: divide-and-conquer
fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    println!("{}", rec(0, n, &a, k));
}
