use std::cmp::*;
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

type Coord = i128; // the type of coordinates

fn half((x, y, _): (Coord, Coord, i32)) -> i32 {
    assert_ne!((x, y), (0, 0));
    if y >= 0 {
        if x > 0 || y > 0 {
            1
        } else {
            2
        }
    } else {
        2
    }
}

// arg sort
// Verified by: https://yukicoder.me/submissions/706856
fn arg_sort(xy: &mut [(Coord, Coord, i32)]) {
    xy.sort_unstable_by(|&a, &b| {
        half(a).cmp(&half(b)).then_with(
            || 0.cmp(&(a.0 * b.1 - a.1 * b.0))
                .then_with(|| a.2.cmp(&b.2)))
    });
}

fn main() {
    input! {
        n: usize,
        a: [(i128, i128); n],
    }
    if n == 1 {
        println!("0");
        return;
    }
    let mut ans = 0;
    for i in 0..n {
        let (xi, yi) = a[i];
        let mut b = vec![];
        for j in 0..n {
            if i != j {
                b.push((a[j].0 - xi, a[j].1 - yi, j as i32));
            }
        }
        arg_sort(&mut b);
        for i in 0..b.len() - 1 {
            let a = b[i];
            let b = b[i + 1];
            if half(a).cmp(&half(b)).then_with(
                || 0.cmp(&(a.0 * b.1 - a.1 * b.0))) == Ordering::Equal {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
