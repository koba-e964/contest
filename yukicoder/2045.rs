use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn perm_comp(a: &[usize], b: &[usize]) -> Vec<usize> {
    let n = a.len();
    assert_eq!(b.len(), n);
    let mut c = vec![0; n];
    for i in 0..n {
        c[i] = a[b[i]];
    }
    c
}

fn perm_exp(a: &[usize], mut e: i64) -> Vec<usize> {
    let n = a.len();
    let mut cur: Vec<_> = (0..n).collect();
    let mut prod = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = perm_comp(&cur, &prod)
        }
        cur = perm_comp(&cur, &cur);
        e /= 2;
    }
    prod
}

// [2, 4, 0, 1, 3, 7, 6] ==> [[0, 2], [1, 4, 3], [6, 7]]
// Verified by: https://atcoder.jp/contests/joisc2007/submissions/24248388
fn decompose_into_cycles(a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut vis = vec![false; n];
    let mut ans = vec![];
    for i in 0..n {
        if vis[i] { continue; }
        vis[i] = true;
        let mut cyc = vec![i];
        let mut v = a[i];
        while v != i {
            vis[v] = true;
            cyc.push(v);
            v = a[v];
        }
        ans.push(cyc)
    }
    ans
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

// https://yukicoder.me/problems/no/2045 (3.5)
// GAP で実験したところ、位数はおよそ O(n^2) でありそうなことがわかった。
// 操作 1 を a, 操作 2 を b と呼ぶことにする。<a, b> の元は (ab)^?, (ab)^?a, (ba)^?, (ba)^?b の形に限られる。
// ここで、ab が有限位数であること、ba = (ab)^{-1} であることから、(ba)^? と (ab)^? の取り得る値の集合は等しいことがわかる。
// よって、(ab)^?, (ab)^?a だけ考えれば良い。
// 答えは ord(ab) * (1 or 2) である。1 or 2 が 2 であることは a = (ab)^i なる i が存在することと同値。
// a の位数は 1 or 2 である。a の位数が 2 のとき、ab^{ord(ab)/2} と a が等しいかどうか比較すれば良い。
// なお、<a,b> の位数は O(n^2) 程度と思われるので素因数分解などは必要なく、i64 で計算して最後に mod を取れば良いようである。
// Tags: group-theory, permutation-groups
fn main() {
    let n: usize = get();
    let p: usize = get();
    let q: usize = get();
    let mut a: Vec<_> = (0..n).collect();
    let mut ab: Vec<_> = (0..n).collect();
    a[..p].reverse();
    ab[..p].reverse();
    ab[n - q..].reverse();
    let cyc = decompose_into_cycles(&ab);
    let mut ord = 1i64;
    let mut is_even = false;
    for c in cyc {
        let len = c.len() as i64;
        let g = gcd(ord, len);
        ord = ord / g * len;
        if len % 2 == 0 {
            is_even = true;
        }
    }
    let mut fac = if p == 1 { 1 } else { 2 };
    if is_even {
        let abx = perm_exp(&ab, ord / 2);
        if a == abx {
            fac = 1;
        }
    }
    println!("{}", ord * fac % 998_244_353);
}
