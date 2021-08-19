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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// Tags: exponentiation-by-squaring, pruning
// Ref: http://oeis.org/A003313
// Ref: http://wwwhomes.uni-bielefeld.de/achim/addition_chain.html
fn main() {
    input!(m: usize, n: usize);
    if m != 1 {
        println!("*");
        return;
    }
    let w = 5;
    let mut dp = vec![vec![vec![]; 1 << w]; max(n, w) + 1];
    for i in 1..w + 1 {
        for bits in 1 << (i - 1)..1 << i {
            let mut ok = (bits & 1) == 1;
            for j in 2..i + 1 {
                if (bits & 1 << (j - 1)) == 0 {
                    continue;
                }
                let found = (1..j).any(
                    |k| (bits & 1 << (k - 1)) != 0 &&
                        (bits & 1 << (j - k - 1)) != 0);
                if !found {
                    ok = false;
                    break;
                }
            }
            if ok {
                let mut tmp = vec![];
                for j in 1..i + 1 {
                    if (bits & 1 << (j - 1)) != 0 {
                        tmp.push(j);
                    }
                }
                dp[i][bits] = tmp;
            }
        }
    }
    for i in w + 1..n + 1 {
        for bits in 0..1 << w {
            for bits2 in 0..1 << w {
                let mut mi = if dp[i][bits | bits2].is_empty() {
                    (100, vec![])
                } else {
                    (dp[i][bits | bits2].len(), dp[i][bits | bits2].clone())
                };
                for j in (1..i / 2 + 1).rev() {
                    if dp[j][bits].is_empty() || dp[i - j][bits2].is_empty() {
                        continue;
                    }
                    let mut tmp = dp[j][bits].clone();
                    tmp.extend(&dp[i - j][bits2]);
                    tmp.push(i);
                    tmp.sort_unstable(); tmp.dedup();
                    if mi.0 > tmp.len() {
                        mi = (tmp.len(), tmp);
                    }
                }
                dp[i][bits | bits2] = mi.1;
            }
        }
    }
    let mut mi = (100, vec![]);
    for i in 0..1 << w {
        if !dp[n][i].is_empty() {
            mi.chmin((dp[n][i].len(), dp[n][i].to_vec()));
        }
    }
    eprintln!("mi = {:?}", mi);
    let ans = mi.1;
    for &v in &ans {
        if v == 1 {
            continue;
        }
        let mut tmp = (0, 0);
        'outer:
        for i in 0..ans.len() {
            for j in 0..i + 1 {
                if ans[i] + ans[j] == v {
                    tmp = (ans[j], ans[i]);
                    break 'outer;
                }
            }
        }
        println!("1 {}\n1 {}", tmp.0, tmp.1);
    }
}
