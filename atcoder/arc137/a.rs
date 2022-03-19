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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn main() {
    input!(l: i64, r: i64);
    let mut d = r - l;
    while d > 0 {
        let rx = r - d;
        let mut ok = false;
        for i in l..rx + 1 {
            if gcd(i, d) == 1 {
                ok = true;
                break;
            }
        }
        if ok {
            println!("{}", d);
            return;
        }
        d -= 1;
    }
    unreachable!();
}
