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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input!(n: usize, m: usize, v: i64);
    let mut ans = 0i64;
    for r in 1..n + 1 {
        if (2 * v) % (r as i64) != 0 {
            continue;
        }
        let t = (2 * v) / r as i64;
        for w in 1..m + 1 {
            if t % (w as i64) != 0 {
                continue;
            }
            let rest = t / w as i64;
            // rest = (a + b)M + c + d
            let rest = rest - (w as i64 - 1) - m as i64 * (r as i64 - 1);
            // rest = 2a*M+2c
            if rest <= 0 || rest % 2 != 0 {
                continue;
            }
            let a = (rest / 2 - 1) / m as i64;
            let c = (rest / 2 - 1) % m as i64;
            if a < ((n + 1 - r) as i64) && c < ((m + 1 - w) as i64) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
