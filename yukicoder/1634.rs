use std::cmp::*;
use std::collections::*;
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

fn dfs2(x: [i64; 9], a: i64, k: i64, ans: &mut HashMap<i64, i64>) {
    if x.iter().all(|&x| x == 0) {
        *ans.entry(a).or_insert(0) += 1;
        return;
    }
    for i in 0..9 {
        if x[i] > 0 {
            let mut y = x;
            y[i] -= 1;
            dfs2(y, (10 * a + i as i64 + 1) % k, k, ans);
        }
    }
}

fn all_perm(x: [i64; 9], k: i64) -> HashMap<i64, i64> {
    let mut ans = HashMap::new();
    dfs2(x, 0, k, &mut ans);
    ans
}

fn calc(hi: [i64; 9], lo: [i64; 9], bias: i64, k: i64) -> i64 {
    let fst = all_perm(hi, k);
    let snd = all_perm(lo, k);
    let mut ans = 0;
    for (k1, v) in fst {
        let k2 = (k - k1) * bias % k;
        ans += *snd.get(&k2).unwrap_or(&0) * v;
    }
    ans
}

fn dfs(v: usize, x: i64, c: &[i64], k: i64, picked: [i64; 9]) -> i64 {
    if v >= 9 {
        if x != 0 {
            return 0;
        }
        let mut rest = [0; 9];
        let mut bias = 1;
        for i in 0..9 {
            rest[i] = c[i] - picked[i];
            for _ in 0..picked[i] {
                bias *= 10;
            }
        }
        return calc(rest, picked, bias, k);
    }
    let mut ans = 0;
    for i in 0..min(x, c[v]) + 1 {
        let mut q = picked;
        q[v] = i;
        ans += dfs(v + 1, x - i, c, k, q);
    }
    ans
}

// https://yukicoder.me/problems/no/1631 (2.5)
// https://yukicoder.me/problems/no/1634 (3.5)
// 半分全列挙。14!/7! = 17297280 なので計算量的には問題ないはず。
// Tags: half-enumeration, sqrt-decomposition
fn main() {
    input! {
        n: i64, k: i64,
        c: [i64; 9],
    }
    println!("{}", dfs(0, n / 2, &c, k, [0; 9]));
}
