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

const MOD: i64 = 100_000;

fn main() {
    input! {
        k: usize, r: usize,
        ta: [(i32, usize); r],
    }
    let mut mask = vec![0; k + 1];
    for (t, a) in ta {
        mask[a] |= 1 << (t - 1);
    }
    let mut dp = vec![[0; 4]; k + 1];
    dp[0][3] = 1;
    for i in 1..k + 1 {
        let mut me = [0; 4];
        for j in 1..4 {
            for l in 1..4 {
                if (l & mask[i]) != 0 { continue; }
                if (l | j) != 3 { continue; }
                me[l] = (me[l] + dp[i - 1][j]) % MOD;
            }
        }
        dp[i] = me;
    }
    println!("{}", dp[k][3]);
}
