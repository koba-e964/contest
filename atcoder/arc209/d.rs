use std::collections::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

pub fn calc(mut a: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    if a[0] == -1 || a[0] == 1 {
        for i in 0..n {
            if a[i] == -1 {
                a[i] = 1;
            }
        }
        return a;
    }
    let mut rem = BTreeSet::new();
    for i in 0..n {
        if a[i] == -1 {
            rem.insert(i as i32 + 1);
        }
    }
    if rem.is_empty() {
        return a;
    }
    if rem.len() == 1 {
        let i = *rem.iter().next().unwrap() as usize - 1;
        let mut r = false;
        for j in 0..i {
            if a[j] == i as i32 + 1 {
                r = true;
            }
        }
        if r {
            a[i] = 1;
        } else {
            let mut mi = (i as i32 + 1, i);
            for j in 0..n {
                if j != i {
                    mi = mi.min((a[j], j));
                }
            }
            a[i] = mi.1 as i32 + 1;
        }
        return a;
    }
    let mut one: usize = n;
    for i in 0..n {
        if a[i] == 1 {
            one = i;
            break;
        }
    }
    for i in 0..n {
        if a[i] != -1 {
            let to = a[i] as usize - 1;
            if a[to] >= 1 {
                continue;
            }
            if to < i {
                assert_eq!(a[to], 0);
                if let Some(&mi) = rem.range(i as i32 + 2..).next() {
                    one = one.min(mi as usize - 1);
                    if one == mi as usize - 1 {
                        assert_eq!(a[mi as usize - 1], -1);
                        a[mi as usize - 1] = 1;
                        rem.remove(&mi);
                    }
                }
            } else {
                assert_eq!(a[to], -1);
                a[to] = 1;
                one = one.min(to);
                rem.remove(&(to as i32 + 1));
            }
            rem.remove(&(i as i32 + 1));
            continue;
        }
        if rem.len() <= 1 && one == n {
            one = i;
            a[i] = 1;
            rem.remove(&(i as i32 + 1));
            continue;
        }
        a[i] = 0;
        rem.remove(&(i as i32 + 1));
    }
    for i in 0..n {
        assert_ne!(a[i], -1);
        if a[i] == 0 {
            a[i] = one as i32 + 1;
        }
    }
    a
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        t: usize,
        a: [[i32]; t],
    }
    for a in a {
        let n = a.len();
        let ans = calc(a);
        if n <= 10 {
            eprintln!("ans = {ans:?}");
        }
        let mut b = vec![0; n];
        for i in 0..n {
            b[i] = ans[ans[i] as usize - 1];
        }
        putvec!(b);
    }
}
