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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

const W: usize = 150_001;

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut freq = vec![vec![]; W];
    for i in 0..n {
        freq[a[i]].push(i);
    }
    let mut ans = vec![0; n];
    macro_rules! emit {
        () => {
            puts!("Yes\n");
            for i in 0..n {
                puts!("{}{}", ans[i], if i + 1 == n { "\n" } else { " " });
            }
            return;
        }
    }
    for i in 1..W {
        if freq[i].len() >= 2 {
            ans[freq[i][0]] = i as i32;
            ans[freq[i][1]] = - (i as i32);
            emit!();
        }
    }
    let mut b = a.clone();
    b.sort();
    let mut ans_set = None;
    let m = min(19, b.len());
    let mut dp = vec![0; 1 << m];
    for bits in 0..1 << m {
        let mut sum = 0;
        for i in 0..m {
            if (bits & 1 << i) != 0 {
                sum += b[i] as i32;
            }
        }
        dp[bits] = sum;
    }
    'outer:
    for bits in 1..1 << m {
        if dp[bits] % 2 != 0 {
            continue;
        }
        for sub in subsets(bits) {
            let sum = dp[bits] - dp[sub] * 2;
            if sum == 0 {
                ans_set = Some((sub, bits - sub));
                break 'outer;
            }
        }
    }
    if let Some((s1, s2)) = ans_set {
        for i in 0..m {
            let idx = freq[b[i]][0];
            if (s1 & 1 << i) != 0 {
                ans[idx] = b[i] as i32;
            } else if (s2 & 1 << i) != 0 {
                ans[idx] = -(b[i] as i32);
            }
        }
        emit!();
    } else {
        puts!("No\n");
    }
}
