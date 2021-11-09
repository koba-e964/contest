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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: usize,
        f: [[i64; k]; n],
        d: usize,
        ab: [(usize1, usize1); d],
        init: [usize1; k],
        r: usize,
        me: [(usize, [usize1; k]); r],
    }
    let mut sets = HashMap::new();
    let mut p: Vec<_> = (0..k).collect();
    loop {
        sets.insert(p.clone(), BTreeSet::<(Vec<i64>, usize)>::new());
        if let None = next_permutation(&mut p) {
            break;
        }
    }
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for &(a, b) in &ab {
        g[a].push(b);
        indeg[b] += 1;
    }
    for i in 0..n {
        if indeg[i] == 0 {
            loop {
                let set = sets.get_mut(&p).unwrap();
                let mut to = vec![0; k];
                for j in 0..k {
                    to[j] = f[i][p[j]];
                }
                set.insert((to, i));
                if let None = next_permutation(&mut p) {
                    break;
                }
            }
        }
    }
    let mut ord: Vec<_> = init;
    let mut pos = 0;
    for done in 0..n {
        if pos < r && me[pos].0 == done {
            ord = me[pos].1.clone();
            pos += 1;
        }
        let top = sets[&ord].iter().rev().next().unwrap().1;
        puts!("{}\n", top + 1);
        loop {
            let set = sets.get_mut(&p).unwrap();
            let mut to = vec![0; k];
            for j in 0..k {
                to[j] = f[top][p[j]];
            }
            set.remove(&(to, top));
            if let None = next_permutation(&mut p) {
                break;
            }
        }
        for &w in &g[top] {
            indeg[w] -= 1;
            if indeg[w] == 0 {
                loop {
                    let set = sets.get_mut(&p).unwrap();
                    let mut to = vec![0; k];
                    for j in 0..k {
                        to[j] = f[w][p[j]];
                    }
                    set.insert((to, w));
                    if let None = next_permutation(&mut p) {
                        break;
                    }
                }
            }
        }
    }
}
