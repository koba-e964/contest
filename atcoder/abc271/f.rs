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

fn calc(a: &[Vec<u32>]) -> Vec<HashMap<u32, i64>> {
    let n = a.len();
    let mut dp = vec![vec![HashMap::new(); n]; n];
    dp[0][0].insert(a[0][0], 1);
    for i in 0..n {
        for j in 0..n - i {
            for (k, v) in dp[i][j].clone() {
                if i + 1 < n {
                    *dp[i + 1][j].entry(k ^ a[i + 1][j]).or_insert(0) += v;
                }
                if j + 1 < n {
                    *dp[i][j + 1].entry(k ^ a[i][j + 1]).or_insert(0) += v;
                }
            }
        }
    }
    let mut ans = vec![HashMap::new(); n];
    for i in 0..n {
        ans[i] = dp[i][n - 1 - i].clone();
    }
    ans
}

// Tags: meet-in-the-middle
fn main() {
    input! {
        n: usize,
        a: [[u32; n]; n],
    }
    let fst = calc(&a);
    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = a[n - 1 - i][n - 1 - j];
        }
    }
    let mut snd = calc(&b);
    snd.reverse();
    let mut ans = 0;
    for i in 0..n {
        for (&k, &v) in &fst[i] {
            if let Some(&v2) = snd[i].get(&(k ^ a[i][n - 1 - i])) {
                ans += v * v2;
            }
        }
    }
    println!("{}", ans);
}
