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

// f(x, a) を a の各要素が x 以上になるように単調増加になるまでに必要な操作 1 の回数とする。
// f(-inf, a[i..n]) + f(max a[i..n], a[0..i]) + i + n などが高速に計算できれば良い。
// f(-inf, a[i..n]) は stack で計算できる。
// f(x, a[0..i]) は二分探索で計算できる。
// Tags: stack
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut acc = vec![0; n];
    for i in 0..n - 1 {
        acc[i + 1] = acc[i] + if a[i] > a[i + 1] { 1 } else { 0 };
    }
    let mut acc2 = vec![0; n + 1];
    for i in 0..n {
        acc2[i + 1] = acc2[i] + a[i];
    }
    let mut t1 = vec![0; n];
    {
        let mut st = vec![];
        let mut sum = 0;
        for i in (0..n).rev() {
            let mut mylen = 1;
            while let Some((v, len)) = st.pop() {
                if v <= a[i] {
                    sum -= v * len;
                    mylen += len;
                } else {
                    st.push((v, len));
                    break;
                }
            }
            st.push((a[i], mylen));
            sum += a[i] * mylen;
            t1[i] = sum - acc2[n] + acc2[i];
        }
    }
    let mut t2 = vec![0; n];
    {
        let mut accma = vec![0; n];
        accma[0] = a[0];
        for i in 1..n {
            accma[i] = max(accma[i - 1], a[i]);
        }
        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = acc[i] + accma[i];
        }
        let mut ma = 0;
        for i in (0..n).rev() {
            ma = max(ma, a[i]);
            let idx = accma[..i].lower_bound(&ma);
            t2[i] = acc[i] - acc[idx] + ma * idx as i64 - acc2[i];
        }
    }
    let mut ans = 1i64 << 60;
    for i in 0..n {
        if acc[i] == acc[n - 1] {
            ans = min(ans, t2[i] + i as i64);
        }
        ans = min(ans, t1[i] + t2[i] + i as i64 + n as i64);
    }
    println!("{}", ans);
}
