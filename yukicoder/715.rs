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

// https://yukicoder.me/problems/no/715 (3.5)
// 全部離れていたら Nim なので N の偶奇。離れているグループ同士は干渉しないので、連続する整数ごとにグループを作って Nim で、しかもグループの Grundy 数は長さのみで決まる。DP?
// N = 5 * 10^5 で実験すると、dp[i] (0 <= i <= N) の最大値は 9 だった。また OEIS で調べると http://oeis.org/A002187 と同一であることがわかった。これは 5 個の n を除いて dp[n] = dp[n + 34] を満たす。これが満たされない n の最大値は 51 なので、n <= 51 + 34 = 85 では dp[n] を計算し、それ以外では (n-52) % 34 を使用してより小さい n に帰着する。
// Tags: dawsons-chess, oeis, nim, octal-game
// ref: http://oeis.org/A002187
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut a = a;
    a.sort();
    let lim = 86;
    let mut dp = vec![0; lim];
    dp[1] = 1;
    for i in 2..lim {
        let mut seen = 1 << dp[i - 2];
        for j in 0..i - 2 {
            let x = dp[j] ^ dp[i - 3 - j];
            seen |= 1 << x;
        }
        let mut mex = 0;
        while (seen & 1 << mex) != 0 { mex += 1; }
        dp[i] = mex;
    }
    let gr = |mut a: usize| -> i32 {
        if a >= lim {
            a = (a - lim + 34) % 34 + lim - 34;
        }
        dp[a]
    };
    let mut last = -1;
    let mut c = 0;
    let mut ans = 0;
    for a in a {
        if last + 1 != a {
            ans ^= gr(c);
            c = 1;
        } else {
            c += 1;
        }
        last = a;
    }
    ans ^= gr(c);
    println!("{}", if ans == 0 { "Second" } else { "First" });
}
