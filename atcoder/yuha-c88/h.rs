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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
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

fn calc(t: &mut [Vec<char>], all: &[(usize, usize, usize, i32)],
        s: &[&[char]]) -> bool {
    let n = s.len();
    for i in 0..n {
        if s[i].len() != all[i].2 {
            return false;
        }
    }
    let mut reset = n;
    'outer:
    for i in 0..n {
        let (x, y, _, kind) = all[i];
        if kind == 1 {
            for j in 0..s[i].len() {
                if t[x][y + j] == '.' {
                    t[x][y + j] = s[i][j];
                } else if t[x][y + j] != s[i][j] {
                    reset = i;
                    break 'outer;
                }
            }
        } else {
            for j in 0..s[i].len() {
                if t[x + j][y] == '.' {
                    t[x + j][y] = s[i][j];
                } else if t[x + j][y] != s[i][j] {
                    reset = i;
                    break 'outer;
                }
            }
        }
    }
    if reset == n {
        return true;
    }
    for i in 0..reset + 1 {
        let (x, y, len, kind) = all[i];
        if kind == 1 {
            for j in 0..len {
                t[x][y + j] = '.';
            }
        } else {
            for j in 0..len {
                t[x + j][y] = '.';
            }
        }
    }
    false
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        s: [chars; n],
        m: usize,
        t: [chars; m],
    }
    let mut all = vec![];
    for i in 0..m - 1 {
        for j in 0..m {
            if t[i][j] == '.' && t[i + 1][j] == '.' && (i == 0 || t[i - 1][j] == '#') {
                let mut pos = i + 2;
                while pos < m && t[pos][j] == '.' { pos += 1; }
                all.push((i, j, pos - i, 0 /* vertical */));
            }
        }
    }
    for i in 0..m {
        for j in 0..m - 1 {
            if t[i][j] == '.' && t[i][j + 1] == '.' && (j == 0 || t[i][j - 1] == '#') {
                let mut pos = j + 2;
                while pos < m && t[i][pos] == '.' { pos += 1; }
                all.push((i, j, pos - j, 1 /* horizontal */));
            }
        }
    }
    eprintln!("{:?}", all);
    let mut p: Vec<_> = (0..n).collect();
    let mut t = t;
    loop {
        let mut tmp = vec![&[] as &[_]; n];
        for i in 0..n {
            tmp[i] = &s[p[i]];
        }
        if calc(&mut t, &all, &tmp) {
            puts!("{}\n", m);
            for t in t {
                puts!("{}\n", t.into_iter().collect::<String>());
            }
            return;
        }
        if let None = next_permutation(&mut p) { break; }
    }
}
