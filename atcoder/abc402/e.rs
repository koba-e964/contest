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
        n: usize, x: usize,
        scp: [(f64, usize, f64); n],
    }
    let mut dp = vec![vec![0.0; 1 << n]; x + 1];
    for i in 1..x + 1 {
        for bits in 0..1 << n {
            let mut me = 0.0f64;
            for j in 0..n {
                if (bits & 1 << j) == 0 {
                    continue;
                }
                let (s, c, p) = scp[j];
                if i >= c {
                    me = me.max((dp[i - c][bits ^ 1 << j] + s) * p / 100.0 + (1.0 - p / 100.0) * dp[i - c][bits]);
                }
            }
            dp[i][bits] = me;
        }
    }
    println!("{}", dp[x][(1 << n) - 1]);
}
