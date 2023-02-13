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

// Ported and modified from https://topcoder-g-hatena-ne-jp.jag-icpc.org/spaghetti_source/20120923/1348327542.html
type Val = i64;
const INF: Val = 1 << 62;

fn smawk_min<F: Fn(usize, usize) -> Val>(js: Vec<usize>, ib: usize, ie: usize, lv: usize, f: &F, minima: &mut [usize]) {
    if ib >= ie { return; }

    let id = 1 << lv;
    let len = (ie - 1 - ib) >> lv;
    let mut js2 = vec![];
    let mut i = ib;
    for &jsq in &js {
        while !js2.is_empty() && f(i, js2[js2.len() - 1]) >= f(i, jsq) {
            js2.pop(); i -= id;
        }
        if js2.len() != len {
            js2.push(jsq); i += id;
        }
    }
  
    smawk_min(js2, ib + id, ie, lv + 1, f, minima);

    let mut i = ib;
    let mut q = 0;
    while i < ie {
        let jt = if i + id < ie {
            minima[i + id]
        } else {
            js[js.len() - 1]
        };
        let mut fm = INF;
        while q < js.len() {
            let fq = f(i, js[q]);
            if fm > fq {
                fm = fq;
                minima[i] = js[q];
            }
            if js[q] == jt { break; }
            q += 1;
        }
        i += id * 2;
    }
}

// f: totally monotone
fn row_minima_tm<F: Fn(usize, usize) -> i64>(
    rows: std::ops::Range<usize>,
    cols: std::ops::Range<usize>,
    f: F, minima: &mut [usize],
) {
    smawk_min(cols.collect(), rows.start, rows.end, 0, &f, minima);
}

// Tags: SMAWK-algorithm, totally-monotone, monge
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
        n: usize, m: usize,
        b: [i64; n],
        c: [i64; m],
    }
    let mut b = b;
    b.sort();
    let mut ci = vec![];
    for i in 0..m {
        ci.push((c[i], i));
    }
    ci.sort(); ci.reverse();
    let mut minima = vec![0; m];
    let f = |i: usize, j: usize| -((n - j) as i64) * (ci[i].0 + b[j]);
    row_minima_tm(0..m, 0..n, &f, &mut minima);
    let mut ans = vec![0; m];
    for i in 0..m {
        ans[ci[i].1] = -f(i, minima[i]);
    }
    putvec!(ans);
}
