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

fn main() {
    input! {
        l: i64, n1: usize, n2: usize,
        vl1: [(i32, i64); n1],
        vl2: [(i32, i64); n2],
    }
    let mut ev = vec![];
    let mut now = 0;
    for (x, y) in vl1 {
        ev.push((now, x, 0));
        now += y;
    }
    now = 0;
    for (x, y) in vl2 {
        ev.push((now, x, 1));
        now += y;
    }
    ev.push((now, 0, 0));
    ev.sort();
    let mut t = 0;
    let mut x = 0;
    let mut y = 0;
    let mut ans = 0;
    for (nxt, val, kind) in ev {
        if x == y {
            ans += nxt - t;
        }
        t = nxt;
        if kind == 0 {
            x = val;
        } else {
            y = val;
        }
    }
    println!("{}", ans);
}
