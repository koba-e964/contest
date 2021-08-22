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

fn calc(a: &[i64], pos: usize, x: i64) -> i64 {
    if pos == 0 {
        return x;
    }
    let mut b = vec![];
    for i in 0..a.len() {
        if (a[i] & 1 << (pos - 1)) != 0 {
            b.push(a[i]);
        }
    }
    if b.len() >= 2 {
        calc(&b, pos - 1, x | 1 << (pos - 1))
    } else {
        calc(&a, pos - 1, x)
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{}", calc(&a, 31, 0) * 2);
}
