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

fn main() {
    input! {
        n: usize, l: i64,
        a: [i64; n],
    }
    const B: usize = 18;
    let mut tbl = vec![vec![n; n]; B];
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    for i in 0..n {
        let idx = acc.upper_bound(&(acc[i] + l));
        tbl[0][i] = idx - 1;
    }
    for i in 0..B - 1 {
        for j in 0..n {
            let x = tbl[i][j];
            tbl[i + 1][j] = if x == n { x } else { tbl[i][x] };
        }
    }
    for x in 0..n {
        let mut ans = 0;
        let mut cur = x;
        for i in (0..B).rev() {
            if tbl[i][cur] < n {
                cur = tbl[i][cur];
                ans |= 1 << i;
            }
        }
        println!("{} {}", ans + 1, acc[n] - acc[cur]);
    }
}
