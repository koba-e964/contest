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
    input!(x: i64, a: i64, d: i64, n: i64);
    if d == 0 {
        println!("{}", (x - a).abs());
        return;
    }
    let (mi, ma) = if d < 0 {
        (a + d * (n - 1), a)
    } else {
        (a, a + d * (n - 1))
    };
    if x <= mi {
        println!("{}", mi - x);
    } else if x >= ma {
        println!("{}", x - ma);
    } else {
        let r = (x - mi) % d.abs();
        println!("{}", core::cmp::min(r, d.abs() - r));
    }
}
