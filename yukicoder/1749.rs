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

const MOD: i64 = 998_244_353;

fn main() {
    input! {
        n: usize, m: usize, t: i64,
        st: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n];
    for (s, t) in st {
        g[s].push(t);
        g[t].push(s);
    }
    let mut dp = vec![0; n];
    dp[0] = 1;
    for _ in 0..t {
        let mut ep = vec![0; n];
        for j in 0..n {
            for &w in &g[j] {
                ep[j] += dp[w];
                if ep[j] >= MOD {
                    ep[j] -= MOD;
                }
            }
        }
        dp = ep;
    }
    println!("{}", dp[0]);
}
