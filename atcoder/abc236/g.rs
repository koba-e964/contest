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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn squmul(a: &[u128], b: &[u128]) -> Vec<u128> {
    let n = a.len();
    let mut ret = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if (a[i] & 1 << j) == 0 { continue; }
            ret[i] |= b[j];
        }
    }
    ret
}

fn squpow(a: &[u128], mut e: i64) -> Vec<u128> {
    let n = a.len();
    let mut sum = vec![0; n];
    for i in 0..n { sum[i] = 1 << i; }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur);
        }
        cur = squmul(&cur, &cur);
        e /= 2;
    }
    sum
}

fn dfs(l: usize, r: usize, e: i64, n: usize, uv: &[(usize, usize)],
       vs: u128, x: &mut [i64]) {
    if vs == 0 {
        return;
    }
    if r - l <= 1 {
        if r <= uv.len() {
            for v in 0..n {
                if (vs & 1 << v) != 0 {
                    x[v] = r as i64;
                }
            }
        }
        return;
    }
    let mid = (l + r) / 2;
    let mut mat = vec![0u128; n];
    for i in 0..mid {
        let (x, y) = uv[i];
        mat[x] |= 1 << y;
    }
    let pw = squpow(&mat, e);
    let frm = vs & pw[0];
    let lat = vs & !pw[0];
    dfs(l, mid, e, n, uv, frm, x);
    dfs(mid, r, e, n, uv, lat, x);
}

// Tags: bit-operations, bit-concurrent, bitset
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, t: usize, l: i64,
        uv: [(usize1, usize1); t],
    }
    let mut x = vec![-1; n];
    dfs(0, t + 1, l, n, &uv, (1 << n) - 1, &mut x);
    putvec!(x);
}
