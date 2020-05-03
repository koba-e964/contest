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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        l: usize,
        s: chars,
        t: chars,
    }
    let mut s = s;
    let mut t = t;
    let mut st = s.clone();
    st.extend_from_slice(&t);
    let mut ts = t.clone();
    ts.extend_from_slice(&s);
    if st > ts {
        std::mem::swap(&mut s, &mut t);
    }
    let n1 = s.len();
    let n2 = t.len();
    let g = gcd(n1 as i64, n2 as i64);
    // s^a ++ t^b
    let mut found = -1;
    for b in 0..n1 as i64 / g {
        let rest = l as i64 - n2 as i64 * b;
        if rest >= 0 && rest % n1 as i64 == 0 {
            found = b;
            break;
        }
    }
    assert_ne!(found, -1);
    // s^(n2/g) > t^(n1/g) ?
    let mut repl = false;
    {
        if l as i64 >= n1 as i64 / g * n2 as i64 {
            let mut ss = vec![];
            let mut tt = vec![];
            for _ in 0..n2 as i64 / g {
                ss.extend_from_slice(&s);
            }
            for _ in 0..n1 as i64 / g {
                tt.extend_from_slice(&t);
            }
            repl = ss > tt;
        }
    }
    let mut ans = vec![];
    let mut count = (l as i64 - n2 as i64 * found) / n1 as i64;
    if repl {
        count %= n2 as i64 / g;
    }
    for _ in 0..count {
        ans.extend_from_slice(&s);
    }
    for _ in 0..(l as i64 - count * n1 as i64) / n2 as i64 {
        ans.extend_from_slice(&t);
    }
    assert_eq!(ans.len(), l);
    for c in ans {
        puts!("{}", c);
    }
    puts!("\n");
    
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
