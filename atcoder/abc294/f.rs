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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: PartialOrd> Bisect<T> for [T] {
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
        n: usize, m: usize, k: usize,
        ab: [(f64, f64); n],
        cd: [(f64, f64); m],
    }
    let mut pass = 1.0;
    let mut fail = 0.0;
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; m];
    for _ in 0..40 {
        let mid = (pass + fail) / 2.0;
        for i in 0..n {
            let (x, y) = ab[i];
            a[i] = x - (x + y) * mid;
        }
        for i in 0..m {
            let (x, y) = cd[i];
            b[i] = x - (x + y) * mid;
        }
        a.sort_by(|&x, &y| x.partial_cmp(&y).unwrap());
        b.sort_by(|&x, &y| x.partial_cmp(&y).unwrap());
        let mut count = 0;
        for &a in &a {
            let idx = b.lower_bound(&-a);
            count += m - idx;
        }
        if count < k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass * 100.0);
}
