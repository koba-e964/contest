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

fn calc(x: &[i64]) -> i64 {
    let n = x.len();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + x[i];
    }
    let mut tot = 0;
    for i in 0..n {
        tot += i as i64 * x[i] - acc[i];
    }
    tot
}

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        y: [i64; n],
    }
    let mut x = x;
    let mut y = y;
    x.sort();
    y.sort();
    println!("{}", calc(&x) + calc(&y));
}
