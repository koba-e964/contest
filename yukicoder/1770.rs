use std::cmp::*;
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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// https://yukicoder.me/problems/no/1770 (4)
// 実験すると、last がいかなる値でも負けであるような残り数がほぼ周期 K+1 で現れているように見える。(K = 3 のとき 5, 9, 13, 17, 21, 25, 29, 33, 37, ...、K = 4 のとき 6, 11, 16, 21, 26, 31, 36, ...、K = 5 のとき 8, 14, 21, 27, 34, 40, ... など)
// K+1 が奇数の場合は、普通の N 言ったらダメゲームと同様の戦略をとれるので (K + 1 = a + b となる a = b なる 2 数が存在しないので) 1 + (K+1 の倍数) の場合に負け確定。
// losing な局面は 2K + O(1) ごとに、全 O(K^2) 状態の中で 4K + O(1) 個しか存在しないようであるため、うまく DP を行えば全列挙できる可能性がある。
// 残り a で last が b である状態を (a, b) と表すことにする。また、残りが a で、x != b であるような x に対して (a, x) が winning であることがわかっている状態を [a, b] と表すことにする。(a, b) が losing であるとき、x != b であるような x に対して (a + b, x) が winning であるため、[a + b, b] へと遷移できる。これで ある a, b, c (b != c) に対して [a, b] と [a, c] が両方わかっていれば、(a, x) はすべて winning であることが保証される。単一の b に対してのみ [a, b] がわかっている場合は、(a, b) のみが losing となる。また [a, b] がわかっている b が存在しない場合、(a, x) はすべて losing である。
// それぞれの a に対して [a, b] という知識は最大 2 個までしか意味をなさない (3 個以上あっても 2 個と変わらない) ため、配列などのデータ構造を使って、[a, b] という知識が 1 個以下であるような a を管理すればよい。初期状態は (1, 1), ..., (1, K) である。ほとんどの a に対して (a, b) が losing であるような b は 1 個以下であり、(a, x) が 1 <= x <= K なる x すべてに対して losing であるのであれば、a + 1 <= c <= a + K なる c に対しては [c, b] という知識が 1 個以上存在するため、そのような a は N/K 個程度しか存在しない。よってこれの計算量は O(N) である。
// Tags: game, nim-like
fn main() {
    let n: usize = get();
    let k: usize = get();
    let mut know = vec![vec![]; n + 1];
    for i in 1..n + 1 {
        let l = know[i].len();
        if l == 0 {
            for j in 1..min(n - i, k) + 1 {
                know[i + j].push(j);
            }
        } else if l == 1 {
            let j = know[i][0];
            if i + j <= n {
                know[i + j].push(j);
            }
        }
    }
    let mut seen = false;
    for i in 1..min(n, k + 1) {
        let l = know[n - i].len();
        if l == 0 || (l == 1 && know[n - i][0] == i) {
            println!("{}", i);
            seen = true;
        }
    }
    if !seen {
        println!("0");
    }
}
