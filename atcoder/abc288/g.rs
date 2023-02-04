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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn pow3(n: usize) -> usize {
    let mut x = 1;
    for _ in 0..n {
        x *= 3;
    }
    x
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        a: [i32; pow3(n)],
    }
    let mut a = a;
    for i in 0..n {
        let x = pow3(i);
        let y = pow3(n - i - 1);
        for i in 0..y {
            for j in 0..x {
                let idx = i * 3 * x + j;
                let v1 = a[idx] + a[idx + 2 * x] - a[idx + x];
                let v0 = a[idx] - v1;
                let v2 = a[idx + 2 * x] - v1;
                a[idx] = v0;
                a[idx + x] = v1;
                a[idx + 2 * x] = v2;
            }
        }
    }
    putvec!(a);
}
