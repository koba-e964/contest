use std::cmp::*;
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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = (acc[i] + a[i]) % 3;
    }
    let mut kind = vec![vec![]; 3];
    for i in 0..n + 1 {
        kind[acc[i]].push(i);
    }
    const INF: usize = 1 << 29;
    let mut dp = vec![[INF; 3]; k + 1];
    dp[0][0] = 0;
    for i in 1..k + 1 {
        for j in 0..3 {
            for l in 0..3 {
                if j == l { continue; }
                if dp[i - 1][l] > n {
                    continue;
                }
                let idx = dp[i - 1][l];
                let nxt = kind[j].lower_bound(&idx);
                dp[i][j] = min(dp[i][j], if nxt >= kind[j].len() { INF } else { kind[j][nxt] });
            }
        }
    }
    if dp[k][acc[n]] > n {
        puts!("No\n");
        return;
    }
    puts!("Yes\n");
    let mut ans = vec![];
    let mut cur = n;
    let mut rem = k;
    while rem > 1 {
        let mut nxt = cur;
        while acc[nxt] == acc[cur] || dp[rem - 1][acc[nxt]] > nxt {
            nxt -= 1;
        }
        ans.push(cur - nxt);
        cur = nxt;
        rem -= 1;
    }
    ans.push(cur);
    ans.reverse();
    putvec!(ans);
}
