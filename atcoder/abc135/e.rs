#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn append_even(ans: &mut Vec<(i64, i64)>, x: i64, y: i64, k: i64) {
    let mut cur = *ans.last().unwrap();
    let q = (x + y - cur.0 - cur.1) / k;
    for _ in 0..q {
        cur.0 += k;
        let rem = max(x, cur.0) - x;
        cur.0 -= rem;
        cur.1 += rem;
        ans.push(cur);
    }
}

fn hop2(x: i64, y: i64, k: i64) -> Vec<(i64, i64)> {
    assert!(x >= 0 && y >= 0 && x + y <= 2 * k);
    assert_eq!((x + y) % 2, 0);
    let r = k - (x + y) / 2;
    let i = if x >= y {
        (k - r, -r)
    } else {
        (-r, k - r)
    };
    let ans = vec![(0, 0), i, (x, y)];
    // eprintln!("{} {} {} {:?}", x, y, k, ans);
    ans
}

fn calc(x: i64, y: i64, k: i64) -> Result<Vec<(i64, i64)>, ()> {
    assert!(x >= 0 && y >= 0);
    if (x + y) % k == 0 {
        let mut ans = vec![(0, 0)];
        append_even(&mut ans, x, y, k);
        return Ok(ans);
    }
    let q = (x + y + k - 1) / k;
    let mut q = max(2, q);
    if (q * k - (x + y)) % 2 != 0 {
        q += 1;
    }
    assert_eq!((q * k - (x + y)) % 2, 0);
    let dx = min(x, (q - 2) * k);
    let dy = (q - 2) * k - dx;
    let nx = x - dx;
    let ny = y - dy;
    let mut ans = hop2(nx.abs(), ny.abs(), k);
    for a in ans.iter_mut() {
        if nx < 0 {
            a.0 = -a.0;
        }
        if ny < 0 {
            a.1 = -a.1;
        }
    }
    append_even(&mut ans, x, y, k);
    Ok(ans)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        k: i64,
        x: i64, y: i64,
    }
    if k % 2 == 0 {
        if (x + y) % 2 != 0 {
            puts!("-1\n");
            return;
        }
    }
    let ans = calc(x.abs(), y.abs(), k);
    let mut ans = match ans {
        Ok(a) => a,
        Err(_) => {
            puts!("-1\n");
            return;
        }
    };
    for a in ans.iter_mut() {
        if x < 0 {
            a.0 = -a.0;
        }
        if y < 0 {
            a.1 = -a.1;
        }
    }
    puts!("{}\n", ans.len() - 1);
    for &(x, y) in &ans[1..] {
        puts!("{} {}\n", x, y);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
