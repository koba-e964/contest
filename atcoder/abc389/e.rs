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

// O(n log A)
fn main() {
    input! {
        n: usize, m: i64,
        p: [i64; n],
    }
    let mut le_calc = |x: i64| -> (/*howmany*/ i64, /*price*/ i64) {
        let mut howmany = 0i64;
        let mut price = 0i64;
        for &a in &p {
            let c = x / a;
            let c = (c + 1) / 2;
            howmany = howmany.saturating_add(c);
            price = price.saturating_add(a.saturating_mul(c).saturating_mul(c));
        }
        (howmany, price)
    };
    let mut pass = 0;
    let mut fail = 1i64 << 60;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let (_, price) = le_calc(mid);
        if price <= m {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let (h1, p1) = le_calc(pass);
    println!("{}", h1 + (m - p1) / (pass + 1));
}
