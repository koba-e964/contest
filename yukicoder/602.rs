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
        n: usize,
        d: [i64; n],
        x: i64, y: i64,
    }
    let z = x.abs() + y.abs();
    if z == 0 {
        println!("0");
        return;
    }
    for &d in &d {
        if z == d {
            println!("1");
            return;
        }
    }
    let mut deven = vec![];
    let mut dodd = vec![];
    for &d in &d {
        if d % 2 == 0 {
            deven.push(d);
        } else {
            dodd.push(d);
        }
    }
    deven.sort();
    dodd.sort();
    for &d in &d {
        let lo = (d - z).abs();
        let hi = d + z;
        let targ = if hi % 2 == 1 {
            &dodd
        } else {
            &deven
        };
        let lo = targ.lower_bound(&lo);
        let hi = targ.upper_bound(&hi);
        if lo < hi {
            println!("2");
            return;
        }
    }
    println!("-1");
}
