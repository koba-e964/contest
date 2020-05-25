#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};

const DEBUG: bool = false;
const STRESS: bool = false;

// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/**
 * Returns the least index of elements that are modified, wrapped with Some.
 * If the entire array is reversed, it returns None instead.
 * v's elements must be pairwise distinct.
 */
fn next_permutation<T: Ord>(v: &mut [T]) -> Option<usize> {
    if v.len() == 0 {
        return None;
    }
    let mut tail_dec: usize = 1;
    let n = v.len();
    while tail_dec < n {
        if v[n - tail_dec - 1] > v[n - tail_dec] {
            tail_dec += 1;
        } else {
            break;
        }
    }
    // v[n - tail_dec .. n] is strictly decreasing
    if tail_dec < n {
        let x = n - tail_dec - 1;
        let mut y = n;
        {
            let pivot = &v[x];
            for i in (n - tail_dec .. n).rev() {
                if v[i] > *pivot {
                    y = i;
                    break;
                }
            }
            assert!(y < n);
        }
        v.swap(x, y);
    }
    v[n - tail_dec .. n].reverse();
    if tail_dec < n {
        Some(n - tail_dec - 1)
    } else {
        None
    }
}

fn calc_naive(mut a: Vec<usize>, p: usize, q: usize) -> Option<Vec<usize>> {
    loop {
        if let None = next_permutation(&mut a) {
            return None;
        }
        let pidx = a.iter().position(|&x| x == p).unwrap();
        let qidx = a.iter().position(|&x| x == q).unwrap();
        if pidx < qidx {
            return Some(a);
        }
    }
}

fn calc(mut a: Vec<usize>, p: usize, q: usize) -> Option<Vec<usize>> {
    let n = a.len();
    if let None = next_permutation(&mut a) {
        return None;
    }
    let pidx = a.iter().position(|&x| x == p).unwrap();
    let qidx = a.iter().position(|&x| x == q).unwrap();
    if pidx < qidx {
        return Some(a);
    }
    if DEBUG {
        eprintln!("p,q = {}, {}", pidx, qidx);
    }
    // qidx < pidx. Try to increment a[qidx].
    a[qidx + 1..].sort();
    a[qidx + 1..].reverse();
    if DEBUG {
        eprintln!("a = {:?}", a);
    }
    if let None = next_permutation(&mut a) {
        return None;
    }
    let pidx = a.iter().position(|&x| x == p).unwrap();
    let qidx = a.iter().position(|&x| x == q).unwrap();
    if pidx < qidx {
        return Some(a);
    }
    if p > q {
        // ? ... ? 0 1  ... q x y p ... (increasing from 0)
        if DEBUG {
            eprintln!("pqa = {} {} {:?}", p, q, a);
        }
        // Turns into ? ... ? 0 1  ... x y p q ... 
        for i in qidx..pidx {
            a.swap(i, i + 1);
        }
        Some(a)
    } else {
        // ? ... ? q 0 1 ... p ...  (increasing from 0)
        assert!(p < q);
        if DEBUG {
            eprintln!("pqidx = {} {}, a = {:?}", pidx, qidx, a);
        }
        let mut ma = 0; // max of the rightmost part
        for i in qidx + 1..n {
            ma = max(ma, a[i]);
        }
        if ma > q {
            a[qidx + 1..].reverse();
            next_permutation(&mut a).unwrap();
            return Some(a);
        }
        // We want q to be in the rightmost "sorted" part.
        // When will it happen?
        let mut rem = BTreeSet::new();
        for i in 0..n {
            rem.insert(i);
        }
        let mut poss = vec![];
        for i in 0..qidx {
            rem.remove(&a[i]);
            let mut it = rem.range(a[i] + 1..);
            let mut res = it.next();
            // Ignoring q
            if res == Some(&q) {
                res = it.next();
            }
            if let Some(&to) = res { poss.push((i, to)); }
        }
        if poss.is_empty() {
            return None;
        }
        let &(idx, to) = poss.last().unwrap();
        let toidx = a.iter().position(|&x| x == to).unwrap();
        a.swap(idx, toidx);
        a[idx + 1..].sort();
        Some(a)
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, p: usize1, q: usize1,
        a: [usize1; n],
    }
    if let Some(ans) = calc(a, p, q) {
        for i in 0..n {
            puts!("{}{}", ans[i] + 1, if i + 1 == n { "\n" } else { " " });
        }
    } else {
        puts!("-1\n");
    }
}

fn stress() {
    for n in 2..7 {
        for p in 0..n {
            for q in 0..n {
                if p == q { continue; }
                let mut a: Vec<usize> = (0..n).collect();
                loop {
                    let ans_naive = calc_naive(a.clone(), p, q);
                    let ans = calc(a.clone(), p, q);
                    if ans != ans_naive {
                        eprintln!("a = {:?}, p = {}, q = {}\n ex = {:?}, act = {:?}", a, p, q, ans_naive, ans);
                    }
                    if let None = next_permutation(&mut a) {
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    if STRESS {
        stress();
    } else {
        solve();
    }
}
