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
        p: [usize; n],
        q: [usize; n],
    }
    let mut pinv = vec![0; n + 1];
    for i in 0..n {
        pinv[p[i]] = i;
    }
    let mut div = vec![vec![]; n + 1];
    for i in 1..n + 1 {
        for j in 1..n / i + 1 {
            div[i * j].push(i);
        }
    }
    let mut dpos = vec![vec![]; n];
    for i in 0..n {
        for &v in &div[q[i]] {
            dpos[i].push(pinv[v] + 1);
        }
        dpos[i].sort_unstable();
        dpos[i].reverse();
    }
    const INF: usize = 1 << 20;
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        for &v in &dpos[i] {
            let idx = dp.lower_bound(&v);
            if idx <= n && dp[idx] > v {
                dp[idx] = v;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n + 1 {
        if dp[i] < INF {
            ans = i;
        }
    }
    println!("{}", ans);
}
