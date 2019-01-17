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

// This solution is written after the author read the editorial.
/*
 * Z algorithm. Calculates an array a[i] = |lcp(s, s[i...|s|])|,
 * where s is the given string.
 * If n = s.length(), the returned array has length n + 1.
 * E.g. z_algorithm("ababa") = {5, 0, 3, 0, 1, 0}
 * Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
 * Verified by: AtCoder ARC055-C (http://arc055.contest.atcoder.jp/submissions/1061771)
 */
fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n + 1];
    ret[0] = n;
    let mut i = 1; let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] { j += 1; }
        ret[i] = j;
        if j == 0 { i += 1; continue; }
        let mut k = 1;
        while i + k < n && k + ret[k] < j {
            ret[i + k] = ret[k];
            k += 1;
        }
        i += k; j -= k;
    }
    ret
}

fn calc_poss(s: &[Vec<char>], k: usize) -> Vec<Vec<bool>> {
    let n = s.len();
    let mut poss = vec![vec![false; k + 1]; n + 1];
    poss[n][0] = true;
    for i in (0 .. n).rev() {
        let len = s[i].len();
        for j in 0 .. k - len + 1 {
            poss[i][j + len] |= poss[i + 1][j];
        }
        for j in 0 .. k + 1 {
            poss[i][j] |= poss[i + 1][j];
        }
    }
    poss
}

fn solve() {
    use std::cmp::Ordering::*;
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        k: usize,
        s: [chars; n],
    }
    let poss = calc_poss(&s, k);
    let mut cs = vec![Vec::new(); n + 1];
    let mut dp = vec![vec![false; k + 1]; n + 1];
    dp[0][0] = true;
    for i in 0 .. n {
        let len = s[i].len();
        let mut css = s[i].clone();
        css.extend_from_slice(&cs[i]);
        let zarr = z_algorithm(&css);
        // cs[i + 1] is implicitly represented by
        // cs[i][..cs_len] + if s_appended { s[i] } else { "" },
        // where (cs_len, s_appended) = cs_next.
        let mut cs_next = (0, false);
        // compares css[a .. a + blen] and css[b .. b + blen].
        // Returns (comparison, is_prefix).
        // If one is a prefix of the other, the longer one is smaller.
        // Either a or b must be 0.
        let cmp_slice = |a: usize, alen: usize, b: usize, blen: usize| {
            let common_len = match (a, b) {
                (0, _) => zarr[b],
                (_, 0) => zarr[a],
                _ => unreachable!(),
            };
            if min(alen, blen) <= common_len {
                (blen.cmp(&alen), true) // longer is smaller
            } else {
                (css[a + common_len].cmp(&css[b + common_len]), false)
            }
        };
        // Compares cs[i][..j] + (if fappend { s[i] } else { "" }) and the current cs[i + 1].
        // If one is a prefix of the other, the longer one wins (is smaller).
        let cmps = |(j, fappend): (usize, bool),
        (cs_len, s_appended): (usize, bool)| {
            match (fappend, s_appended) {
                (true, true) if j == cs_len => (Equal, true),
                (true, true) if j > cs_len + len =>
                    cmp_slice(len + cs_len, j - cs_len, 0, len),
                (true, true) if cs_len > j + len =>
                    cmp_slice(0, len, len + j, cs_len - j),
                (true, true) if j > cs_len => {
                    let res = cmp_slice(len + cs_len, j - cs_len, 0, j - cs_len);
                    if res.0 != Equal { return res }
                    cmp_slice(0, len, j - cs_len, len - (j - cs_len))
                },
                (true, true) if cs_len > j => {
                    let res = cmp_slice(0, cs_len - j, len + j, cs_len - j);
                    if res.0 != Equal { return res }
                    cmp_slice(cs_len - j, len - (cs_len - j), 0, len)
                },
                (true, true) => unreachable!(),

                (true, false) if cs_len <= j => (Less, true),
                (true, false) => cmp_slice(0, len, len + j, cs_len - j),

                (false, true) if j <= cs_len => (Greater, true),
                (false, true) => cmp_slice(len + cs_len, j - cs_len, 0, len),

                (false, false) => (cs_len.cmp(&j), true),
            }
        };
        // get min
        for j in 0 .. k + 1 {
            if !poss[i + 1][k - j] || !dp[i][j] { continue; }
            if cmps((j, false), cs_next).0 == Less {
                cs_next = (j, false);
            }
        }
        for j in 0 .. k - len + 1 {
            if !poss[i + 1][k - j - len] || !dp[i][j] { continue; }
            if cmps((j, true), cs_next).0 == Less {
                cs_next = (j, true);
            }
        }
        // update dp
        for j in 0 .. k + 1 {
            if !poss[i + 1][k - j] || !dp[i][j] { continue; }
            dp[i + 1][j] |= cmps((j, false), cs_next).1;
        }
        for j in 0 .. k - len + 1 {
            if !poss[i + 1][k - j - len] || !dp[i][j] { continue; }
            dp[i + 1][j + len] |= cmps((j, true), cs_next).1;
        }
        cs[i + 1] = cs[i][..cs_next.0].to_vec();
        if cs_next.1 {
            cs[i + 1].extend_from_slice(&s[i]);
        }
    }

    puts!("{}\n", cs[n][..k].iter().cloned().collect::<String>());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
