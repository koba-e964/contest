use std::io::{Write, BufWriter};
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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        _n: i64, m: i64,
        q: usize,
        abcd: [(i64, i64, i64, i64); q],
    }
    let get = |x: i64, y: i64| {
        let even = (y * (y + 1) / 2) % MOD;
        let even = (even + (y / 2) * m) % MOD;
        let mut tmp = even * (x / 2) % MOD;
        let val = (x / 2) * (x / 2 - 1) % MOD;
        tmp += m * val % MOD * y;
        tmp %= MOD;
        if x % 2 == 1 {
            tmp += ((y + 1) / 2) * m % MOD * (x - 1) % MOD;
            tmp += ((y + 1) / 2) * ((y + 1) / 2) % MOD;
        }
        tmp % MOD
    };
    for (a, b, c, d) in abcd {
        let val = get(b, d) - get(a - 1, d) - get(b, c - 1) + get(a - 1, c - 1) + 2 * MOD;
        puts!("{}\n", val % MOD);
    }
}
