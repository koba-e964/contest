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

// Tags: geometry
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut ma = 0;
    let set: std::collections::HashSet<_> = xy.iter().cloned().collect();
    for i in 0..n {
        let (xi, yi) = xy[i];
        for j in 0..n {
            let (xj, yj) = xy[j];
            let dx = xj - xi;
            let dy = yj - yi;
            let xk = xj - dy;
            let yk = yj + dx;
            let xl = xi - dy;
            let yl = yi + dx;
            if set.contains(&(xk, yk)) && set.contains(&(xl, yl)) {
                ma = std::cmp::max(ma, dx * dx + dy * dy);
            }
        }
    }
    println!("{}", ma);
}
