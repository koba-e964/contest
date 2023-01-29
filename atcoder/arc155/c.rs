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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn solve(a: Vec<i64>, b: Vec<i64>) -> bool {
    if a == b {
        return true;
    }
    let n = a.len();
    {
        let mut aa = a.clone();
        let mut bb = b.clone();
        aa.sort();
        bb.sort();
        if aa != bb {
            return false;
        }
    }
    if a.iter().all(|&x| x % 2 == 1) {
        return false;
    }
    if a.iter().filter(|&x| x % 2 == 0).count() <= 2 {
        let aa: Vec<_> = a.iter().filter(|&x| x % 2 == 0).collect();
        let bb: Vec<_> = b.iter().filter(|&x| x % 2 == 0).collect();
        return aa == bb;
    }
    let mut near_a = false;
    let mut near_b = false;
    for i in 0..n {
        if a[i] % 2 == 0 { continue; }
        for j in i + 1..min(n, i + 3) {
            if a[j] % 2 == 1 {
                near_a = true;
                break;
            }
        }
    }
    for i in 0..n {
        if b[i] % 2 == 0 { continue; }
        for j in i + 1..min(n, i + 3) {
            if b[j] % 2 == 1 {
                near_b = true;
                break;
            }
        }
    }
    if near_a != near_b {
        return false;
    }
    if near_a {
        return true;
    }
    let mut odd = vec![0];
    for i in 0..n {
        if a[i] % 2 == 1 {
            if a[i] != b[i] {
                return false;
            }
            odd.push(i + 1);
        }
    }
    odd.push(n + 1);
    for i in 0..odd.len() - 1 {
        let x = odd[i];
        let y = odd[i + 1] - 1;
        let mut aa = a[x..y].to_vec();
        let mut bb = b[x..y].to_vec();
        if y - x <= 2 {
            if aa != bb {
                return false;
            }
        }
        aa.sort();
        bb.sort();
        if aa != bb {
            return false;
        }
    }
    true
}

// Solved with hints
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    println!("{}", if solve(a, b) { "Yes" } else { "No" });
}
