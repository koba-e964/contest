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
        n: usize,
        tab: [(i32, usize, usize); n - 1],
        q: usize,
        x: [i64; q],
    }
    for x in x {
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = x;
        for i in 2..n + 1 {
            let (t, a, b) = tab[i - 2];
            dp[i] = match t {
                1 => dp[a] + dp[b],
                2 => a as i64 * dp[b],
                3 => dp[a] * dp[b],
                _ => panic!(),
            } % MOD;
        }
        println!("{}", dp[n]);
    }
}
