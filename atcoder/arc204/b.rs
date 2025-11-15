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

// [2, 4, 0, 1, 3, 7, 6] ==> [[0, 2], [1, 4, 3], [6, 7]]
// Verified by: https://atcoder.jp/contests/joisc2007/submissions/24248388
fn decompose_into_cycles(a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut vis = vec![false; n];
    let mut ans = vec![];
    for i in 0..n {
        if vis[i] { continue; }
        vis[i] = true;
        let mut cyc = vec![i];
        let mut v = a[i];
        while v != i {
            vis[v] = true;
            cyc.push(v);
            v = a[v];
        }
        ans.push(cyc)
    }
    ans
}

fn f(a: &[usize], n: usize) ->  usize {
    let m = a.len();
    let mut conn = vec![vec![]; m];
    for i in 0..m {
        for j in i + 1..m {
            if a[i] % n == a[j] % n {
                conn[i].push(j);
            }
        }
    }
    let mut dp = vec![vec![0; m + 1]; m + 1];
    for s in 2..m + 1 {
        for l in 0..=m - s {
            let r = l + s;
            let mut me = dp[l + 1][r];
            for &to in &conn[l] {
                if to < r {
                    me = me.max(dp[l + 1][to + 1] + dp[to + 1][r] + 1);
                }
            }
            dp[l][r] = me;
        }
    }
    dp[0][m]
}

pub fn calc(p: Vec<usize>, n: usize, _k: usize) -> usize {
    let cycs = decompose_into_cycles(&p);
    //eprintln!("cycs = {cycs:?}");
    let mut ans = 0;
    for cyc in cycs {
        if cyc.is_empty() { continue; }
        ans += f(&cyc, n);
    }
    ans
}

fn main() {
    input! {
        n: usize, k: usize,
        p: [usize1; n * k],
    }
    println!("{}", calc(p, n, k));
}
