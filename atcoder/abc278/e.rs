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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize, n: usize, x: usize, y: usize,
        a: [[usize; w]; h],
    }
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    let mut acc = vec![vec![vec![0; n]; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            for k in 0..n {
                acc[i + 1][j + 1][k] = acc[i + 1][j][k] + acc[i][j + 1][k] - acc[i][j][k];
            }
            acc[i + 1][j + 1][a[i][j] - 1] += 1;
        }
    }
    for i in 0..h - x + 1 {
        let mut v = vec![0; w - y + 1];
        for j in 0..w - y + 1 {
            v[j] = (0..n).filter(|&k| acc[i + x][j + y][k] - acc[i + x][j][k] - acc[i][j + y][k] + acc[i][j][k] < acc[h][w][k])
                .count();
        }
        putvec!(v);
    }
}
