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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn reach(s: i64, t: i64, a: i64, b: i64, odd: bool) -> bool {
    if a == b {
        return if odd { s + t == 2 * a } else { s == t };
    }
    (s + t) % 2 == 0
}

const LIM: usize = 800_000;

fn emit(mut s: i64, t: i64, a: i64, b: i64, odd: bool) -> Vec<i64> {
    if a == b {
        return if odd { vec![a; LIM + 1] } else { vec![a; LIM] };
    }
    let mut op = vec![];
    if odd {
        s = 2 * a - s;
        op.push(a);
    }
    while s > t {
        op.push(a + 1);
        op.push(a);
        s -= 2;
    }
    while s < t {
        op.push(a);
        op.push(a + 1);
        s += 2;
    }
    while op.len() < LIM {
        op.push(a);
        op.push(a);
    }
    op
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        sx: i64, sy: i64,
        tx: i64, ty: i64,
        a: i64, b: i64, c: i64, d: i64,
    }
    for &t in &[false, true] {
        if reach(sx, tx, a, b, t) && reach(sy, ty, c, d, t) {
            let op1 = emit(sx, tx, a, b, t);
            let op2 = emit(sy, ty, c, d, t);
            puts!("Yes\n");
            for i in 0..op1.len() {
                puts!("{} {}\n", op1[i], op2[i]);
            }
            return;
        }
    }
    puts!("No\n");
}
