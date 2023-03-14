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

// Tags: mos-algorithm
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [usize; n],
        lr: [(usize1, usize); q],
    }
    const W: usize = 200_100;
    const B: usize = 500;
    let mut lri: Vec<_> = (0..q).map(|i| {
        let (l, r) = lr[i];
        (l, r, i)
    }).collect();
    lri.sort_by_key(|&(l, r, _idx)| {
        let q = l / B;
        if q % 2 == 1 {
            (q, n - r)
        } else {
            (q, r)
        }
    });
    let mut ans = vec![0; q];

    // pointer
    let mut cl = 0;
    let mut cr = 0;

    // state
    let mut sum = 0i64;
    let mut v = vec![0i64; W];

    macro_rules! add {
        ($v:expr) => {
            let idx = $v;
            v[idx] += 1;
            sum += (v[idx] - 1) * (v[idx] - 2) / 2;
        }
    }
    macro_rules! sub {
        ($v:expr) => {
            let idx = $v;
            sum -= (v[idx] - 1) * (v[idx] - 2) / 2;
            v[idx] -= 1;
        }
    }
    for &(l, r, idx) in &lri {
        while cr < r {
            add!(a[cr]);
            cr += 1;
        }
        while cl > l {
            cl -= 1;
            add!(a[cl]);
        }
        while cr > r {
            cr -= 1;
            sub!(a[cr]);
        }
        while cl < l {
            sub!(a[cl]);
            cl += 1;
        }
        ans[idx] = sum;
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
