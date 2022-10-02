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

fn main() {
    input! {
        n: usize, s: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[n][0] = true;
    for i in (0..n).rev() {
        let (a, b) = ab[i];
        for j in a..s + 1 {
            dp[i][j] |= dp[i + 1][j - a];
        }
        for j in b..s + 1 {
            dp[i][j] |= dp[i + 1][j - b];
        }
    }
    if !dp[0][s] {
        println!("No");
        return;
    }
    println!("Yes");
    let mut ans = "".to_string();
    let mut rem = s;
    for i in 0..n {
        let (a, b) = ab[i];
        if rem >= a && dp[i + 1][rem - a] {
            ans.push('H');
            rem -= a;
            continue;
        }
        ans.push('T');
        rem -= b;
    }
    println!("{}", ans);
}
