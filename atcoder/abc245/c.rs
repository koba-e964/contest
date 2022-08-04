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

fn main() {
    input! {
        n: usize, k: i64,
        a: [[i64; n]; 2],
    }
    let mut dp = vec![[false; 2]; n];
    dp[0] = [true; 2];
    for i in 1..n {
        for j in 0..2 {
            for l in 0..2 {
                if dp[i - 1][j] && (a[j][i - 1] - a[l][i]).abs() <= k {
                    dp[i][l] = true;
                }
            }
        }
    }
    println!("{}", if dp[n - 1][0] || dp[n - 1][1] { "Yes" } else { "No" });
}
