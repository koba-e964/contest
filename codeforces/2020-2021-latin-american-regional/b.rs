use std::cmp::*;
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

/**
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, both min and gcd satisfy these properties.)
 * Verified by: AtCoder CODE FESTIVAL 2016 Tournament Round 3 (Parallel) B
 * (http://cf16-tournament-round3-open.contest.atcoder.jp/submissions/1026294)
 */
struct SparseTable<T, BiOp> {
    biop: BiOp,
    st: Vec<Vec<T>>,
}

impl<T, BiOp> SparseTable<T, BiOp>
    where BiOp: Fn(T, T) -> T,
          T: Copy {
    pub fn new(ary: &[T], biop: BiOp) -> Self {
        let n = ary.len();
        let mut h = 1;
        while 1 << h < n {
            h += 1;
        }
        let mut st: Vec<Vec<T>> = vec![Vec::from(ary); h + 1];
        for i in 0 .. n {
            st[0][i] = ary[i];
        }
        for b in 1 .. (h + 1) {
            if n + 1 < 1 << b {
                break;
            }
            for i in 0 .. (n + 1 - (1 << b)) {
                let next_idx = (1 << (b - 1)) + i;
                st[b][i] = biop(st[b - 1][i], st[b - 1][next_idx]);
            }
        }
        SparseTable {biop: biop, st: st}
    }
    fn top_bit(t: usize) -> usize {
        8 * std::mem::size_of::<usize>() - 1 - t.leading_zeros() as usize
    }
    pub fn query(&self, f: usize, s: usize) -> T {
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        (self.biop)(self.st[b][f], self.st[b][endpoint])
    }
}

// Tags: range-max-query, range-min-query, dropping-log
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ai = vec![(0, 0); n];
    for i in 0..n {
        ai[i] = (a[i], i);
    }
    let spta = SparseTable::new(&ai, max);
    let mut cave_l = vec![0; n];
    let mut cave_r = vec![n + 1; n];
    for i in 0..n {
        if a[i] == -1 {
            continue;
        }
        let mut pass = i;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if spta.query(i, mid).0 > a[i] {
                fail = mid;
            } else {
                pass = mid;
            }
        }
        cave_r[i] = fail;
        pass = i + 1;
        fail = 0;
        while pass - fail > 1 {
            let mid = (fail + pass) / 2;
            if spta.query(mid - 1, i).0 > a[i] {
                fail = mid;
            } else {
                pass = mid;
            }
        }
        cave_l[i] = fail;
    }
    let cave_l = SparseTable::new(&cave_l, max);
    let cave_r = SparseTable::new(&cave_r, min);
    for l in 3..n + 1 {
        if n % l <= 2 && n % l >= 1 {
            continue;
        }
        let mut ok = true;
        for i in 0..(n + l - 1) / l {
            let x = i * l;
            let y = min(i * l + l, n);
            let cent = spta.query(x, y - 1).1;
            let lft = cave_l.query(x, cent);
            let rgt = cave_r.query(cent, y - 1);
            if x < lft || rgt < y
                || (a[x + 1] != -1 && a[x] != -1 && a[x] > a[x + 1])
                || (a[y - 2] != -1 && a[y - 1] != -1 && a[y - 1] > a[y - 2]) {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Y");
            return;
        }
    }
    println!("N");
}
