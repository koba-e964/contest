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

struct SqrtDecomp {
    bl: usize,
    a: Vec<i64>,
    b: Vec<i64>,
}

impl SqrtDecomp {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m * m < n {
            m += 1;
        }
        SqrtDecomp {
            bl: m,
            a: vec![0; n],
            b: vec![0; (n + m - 1) / m],
        }
    }
    fn add(&mut self, idx: usize, x: i64) {
        self.a[idx] += x;
        self.b[idx / self.bl] += x;
    }
    fn query(&self, l: usize, r: usize) -> i64 {
        let bl = self.bl;
        if r - l <= bl {
            let mut tot = 0;
            for i in l..r {
                tot += self.a[i];
            }
            return tot;
        }
        let lb = (l + bl - 1) / bl;
        let rb = r / bl;
        let mut tot = 0;
        for i in l..lb * bl {
            tot += self.a[i];
        }
        for i in lb..rb {
            tot += self.b[i];
        }
        for i in rb * bl..r {
            tot += self.a[i];
        }
        tot
    }
}

// https://yukicoder.me/problems/no/924 (4)
// Mo で O(Q sqrt(N) log N)。座標圧縮後、座標を平方分割して更新が O(1)、中央値の計算が O(sqrt(N)) でできるようにすると O(Q sqrt(N)) でできる。
// Tags: sqrt-decomposition, mos-algorithm
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        lr: [(usize1, usize); q],
    }
    let mut coo = a.clone();
    coo.sort(); coo.dedup();
    let m = coo.len();
    let a: Vec<usize> = a.iter().map(|a| coo.binary_search(a).unwrap()).collect();
    let mut lri = vec![];
    for i in 0..q {
        let (l, r) = lr[i];
        lri.push((l, r, i));
    }
    lri.sort_unstable_by_key(|&(l, r, _)| {
        let b = l / 450;
        (b, if b % 2 == 0 { r } else { n - r })
    });
    let mut x = 0;
    let mut y = 0;
    let mut sum = SqrtDecomp::new(m);
    let mut cnt = SqrtDecomp::new(m);
    let mut ans = vec![0; q];
    for (l, r, idx) in lri {
        while y < r {
            cnt.add(a[y], 1);
            sum.add(a[y], coo[a[y]]);
            y += 1;
        }
        while x > l {
            x -= 1;
            cnt.add(a[x], 1);
            sum.add(a[x], coo[a[x]]);
        }
        while y > r {
            y -= 1;
            cnt.add(a[y], -1);
            sum.add(a[y], -coo[a[y]]);
        }
        while x < l {
            cnt.add(a[x], -1);
            sum.add(a[x], -coo[a[x]]);
            x += 1;
        }
        let targ = (r - l) as i64 / 2;
        let mut now = 0;
        let mut pos = 0;
        while now <= targ {
            if now + cnt.b[pos] <= targ {
                now += cnt.b[pos];
                pos += 1;
            } else {
                break;
            }
        }
        pos *= cnt.bl;
        while now <= targ {
            if now + cnt.a[pos] <= targ {
                now += cnt.a[pos];
                pos += 1;
            } else {
                break;
            }
        }
        let val = coo[pos];
        ans[idx] = sum.query(pos, m) - sum.query(0, pos) + val * (2 * cnt.query(0, pos) - (r - l) as i64);
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
