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

fn g(a: Vec<i64>) -> i64 {
    let n = a.len() as i64;
    let mut tot = 0;
    for i in 0..n {
        tot += a[i as usize] * (2 * i - n + 1);
    }
    tot
}

fn f(a: &[(i64, i64)]) -> i64 {
    if a.is_empty() {
        return 0;
    }
    let mut bx = vec![];
    let mut by = vec![];
    let c = (a[0].0 + a[0].1) & 1;
    for &(x, y) in a {
        bx.push((x + y - c) / 2);
        by.push((x - y + c) / 2);
    }
    bx.sort(); by.sort();
    g(bx) + g(by)
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut even = vec![];
    let mut odd = vec![];
    for i in 0..n {
        if (xy[i].0 + xy[i].1) % 2 == 0 {
            even.push(xy[i]);
        } else {
            odd.push(xy[i]);
        }
    }
    println!("{}", f(&even) + f(&odd));
}
