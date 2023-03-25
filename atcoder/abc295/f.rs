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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(s: &[char], mut x: i64) -> i64 {
    let mut p = 1i64;
    let mut c = 0i64;
    for i in 0..s.len() {
        c = 10 * c + (s[i] as u8 - b'0') as i64;
        p *= 10;
    }
    if s[0] == '0' {
        c += p;
    }
    let mut ans = 0;
    let this = |mut val: i64| -> i64 {
        let mut ans = 0;
        while val >= c {
            if val % p == c % p {
                ans += 1;
            }
            val /= 10;
        }
        ans
    };
    let mut fac = 1i64;
    while x > c {
        ans += (x - c + p - 1) / p * fac;
        ans += this(x / 10) * (x % 10) * fac;
        x /= 10;
        fac *= 10;
    }
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        slr: [(chars, i64, i64); t],
    }
    for (s, l, r) in slr {
        puts!("{}\n", calc(&s, r + 1) - calc(&s, l));
    }
}
