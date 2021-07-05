use std::io::{Write, BufWriter};
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: construction, permutation
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let mut ops = vec![];
    input! {
        n: usize,
        p: [usize; n],
    }
    if n == 2 {
        if p[0] == 1 {
            puts!("1\n0\n");
        } else {
            puts!("0\n");
        }
        return;
    }
    let mut idx1 = p.iter().position(|&x| x == 1).unwrap();
    let mut idx2 = p.iter().position(|&x| x == 2).unwrap();
    let mut p = p;
    while (idx1 + 1) % n != idx2 {
        let nxt = (idx1 + 1) % n;
        p.swap(idx1, nxt);
        ops.push(idx1);
        idx1 = nxt;
    }
    let mut cur = if idx2 == n - 1 || idx2 == 0 { 2 } else { 0 };
    loop {
        if ops.len() > 200_000 {
            panic!();
        }
        let ok = (0..n - 1).all(|i| p[i] < p[i + 1]);
        if ok {
            break;
        }
        let a = (idx2 + 1) % n;
        let b = (idx2 + 2) % n;
        if b != cur && p[a] > p[b] {
            // 1 2 a b
            p.swap(idx2, b);
            ops.push(idx2);
            // 1 b a 2
            p.swap(idx1, idx2);
            ops.push(idx1);
            // b 1 a 2
            p.swap(idx2, a);
            ops.push(idx2);
            // b a 1 2
            idx1 = a;
            idx2 = b;
            if cur == (b + 1) % n {
                // 1 2 a b | ==> b a | 1 2
                cur = (cur + n - 2) % n;
            }
        } else {
            // 1 2 a b
            p.swap(idx1, idx2);
            ops.push(idx1);
            // 2 1 a b
            p.swap(idx1, a);
            ops.push(idx1);
            // a 1 2 b
            if cur == b {
                // 1 2 a | b ==> a | 1 2 b
                cur = (cur + n - 2) % n;
            }
            idx1 = idx2;
            idx2 = a;
        }
    }
    puts!("{}\n", ops.len());
    for o in ops {
        puts!("{}\n", o);
    }
}
