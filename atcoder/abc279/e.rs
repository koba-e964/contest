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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [usize1; m],
    }
    // snd[i] = s_{i-1} ... s_0(0)
    let mut snd = vec![0; m + 1];
    {
        // p[i] = s_{i-1} ... s_0
        let mut p: Vec<_> = (0..n).collect();
        let mut pinv = p.clone();
        for i in 0..m {
            let x = pinv[a[i]];
            let y = pinv[a[i] + 1];
            p.swap(x, y);
            pinv.swap(a[i], a[i] + 1);
            snd[i + 1] = p[0];
        }
    }
    // ans[i] = s_{m-1} ... s_{i+1} s_{i-1} ... s_0(0)
    //   = s_{m-1} ... s_{i+1}(snd[i])
    let mut ans = vec![0; m];
    let mut p: Vec<_> = (0..n).collect();
    for i in (0..m).rev() {
        ans[i] = p[snd[i]];
        p.swap(a[i], a[i] + 1);
    }
    for i in 0..m {
        puts!("{}\n", ans[i] + 1);
    }
}
