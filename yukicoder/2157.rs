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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

// https://yukicoder.me/problems/no/2157 (3.5)
// 二分探索で O(N^2 log N log (max d))。
fn main() {
    input! {
        n: usize, m: usize,
        d: [[i64; m]; n],
    }
    let mut d = d;
    for i in 0..n {
        d[i].sort_unstable();
    }
    const INF: i64 = 1 << 30;
    let mut pass = INF;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut acc = vec![0; m + 1];
        for i in 0..m {
            acc[i + 1] = acc[i] + 1;
        }
        for i in 1..n {
            let mut bcc = vec![0; m + 1];
            for j in 0..m {
                let lo = d[i - 1].lower_bound(&(d[i][j] - mid));
                let hi = d[i - 1].upper_bound(&d[i][j]);
                bcc[j + 1] = bcc[j] + if acc[lo] < acc[hi] { 1 } else { 0 };
            }
            acc = bcc;
        }
        if acc[m] > 0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", if pass >= INF { -1 } else { pass });
}
