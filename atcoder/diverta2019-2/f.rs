#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
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

/**
 * Returns the least index of elements that are modified, wrapped with Some.
 * If the entire array is reversed, it returns None instead.
 * v's elements must be pairwise distinct.
 */
fn next_permutation<T: Ord>(v: &mut [T]) -> Option<usize> {
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

fn check(n: usize, dist: &[Vec<u64>]) -> Result<(), (usize, Vec<usize>, HashSet<u64>)> {
    let mut seen = HashSet::new();
    let mut p: Vec<_> = (0..n).collect();
    let mut col = 0;
    let mut ret = Ok(());
    loop {
        if p[0] < p[n - 1] {
            let mut tot = 0;
            for i in 0..n - 1 {
                tot += dist[p[i]][p[i + 1]];
            }
            if seen.contains(&tot) || tot > 100_000_000_000 {
                ret = Err(p.clone());
                col += 1;
            }
            seen.insert(tot);
        }
        if let None = next_permutation(&mut p) { break; }
    }
    ret.map_err(|x| (col, x, seen))
}

fn wrap(n: usize, dist: &[Vec<u64>], a: u64, b: u64) -> Vec<Vec<u64>> {
    let seq = [1, 2, 3, 5, 8, 13, 21, 34, 55];
    let mut ret = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            ret[i][j] = dist[i][j] * a + b
        }
    }
    for i in 0..n {
        ret[i][n] = seq[i];
        ret[n][i] = seq[i];
    }
    ret
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input!(n: usize);
    let mut dist = vec![vec![0, 1], vec![1, 0]];
    for i in 2..n {
        if i == 2 {
            dist = vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]];
        } else {
            // These parameters are found by experiments.
            let a = [0, 0, 0, 6, 8, 14, 29, 49, 65, 178][i];
            let b = [0, 0, 0, 3, 8, 11, 21, 30, 43, 89][i];
            dist = wrap(i, &dist, a, b);
        }
    }
    assert!(check(n, &dist).is_ok());
    for i in 0..n {
        for j in 0..n {
            puts!("{}{}", dist[i][j], if j + 1 == n { "\n" } else { " " });
        }
    }
}
