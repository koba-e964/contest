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

// https://yukicoder.me/problems/no/1515 (3.5)
// カード c をめくったときの遷移は dp[a][b] -max-> dp[c][a], dp[a][b] -max-> dp[c][b] という形に限られる。ほとんどの遷移は +0 なので、各行各列の max を記憶しておけば良い。 -> dp[a][b] -> dp[a][b] (めくったカードを捨てる) という遷移もあった。ほとんど無視できるが k | a + b + c の場合には += 1 が必要。
// Tags: dp, inplace-dp, overwriting-dp
fn main() {
    input! {
        n: usize, k: usize, x: usize, y: usize,
        a: [usize; n],
    }
    const INF: i32 = 1 << 28;
    let mut dp = vec![vec![-INF; k]; k];
    dp[x % k][y % k] = 0;
    let mut row = vec![-INF; k];
    let mut col = vec![-INF; k];
    row[x % k] = 0;
    col[y % k] = 0;
    for &a in &a {
        let a = a % k;
        let mut delay = vec![];
        for b in 0..k {
            let rest = (2 * k - a - b) % k;
            delay.push((b, rest, dp[b][rest] + 1));
        }
        for b in 0..k {
            let mut ma = max(row[b], col[b]);
            let rest = (2 * k - a - b) % k;
            ma = max(ma, max(dp[rest][b], dp[b][rest]) + 1);
            delay.push((a, b, ma));
        }
        for (i, j, val) in delay {
            dp[i][j] = max(dp[i][j], val);
            row[i] = max(row[i], dp[i][j]);
            col[j] = max(col[j], dp[i][j]);
        }
    }
    let ans = row.iter().max().unwrap();
    println!("{}", ans);
}
