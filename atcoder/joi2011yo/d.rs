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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: dp, overflow
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp = vec![[0u64; 21]; n];
    dp[1][a[0]] = 1;
    for i in 1..n - 1 {
        for j in 0..21 {
            if j >= a[i] {
                dp[i + 1][j] = dp[i + 1][j].wrapping_add(dp[i][j - a[i]]);
            }
            if j + a[i] <= 20 {
                dp[i + 1][j] = dp[i + 1][j].wrapping_add(dp[i][j + a[i]]);
            }
        }
    }
    println!("{}", dp[n - 1][a[n - 1]]);
}
