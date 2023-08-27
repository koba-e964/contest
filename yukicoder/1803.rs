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

// https://yukicoder.me/problems/no/1803 (3.5)
// 問題のコスト関数を重みとした完全グラフの最小全域木の重みが答え。M < N のとき、i と M - i、M - i と M + i がコスト 0 で繋がるので、i と M + i がコスト 0 で繋がる。(1 <= i <= N - M)
// M - i と i が繋がることから、頂点番号が M / 2 よりも大きい頂点は、自分未満の番号の頂点のいずれかと繋がるので、代表としては頂点番号 M / 2 以下のものだけを考えれば良い。(例外は M)
// またコストを考えるとき、使う頂点としては i と M-i のみを考えれば良い。なぜなら M+i を使うときのコストと i を使うときのコストは同じであるため。
// N >= M であれば、コスト 0 で繋げられる頂点を繋げた後の連結成分の個数は floor(M / 2) + 1 個。
// このとき (1, M), (2, M - 1), ... と繋ぐことでコスト floor(M / 2) で連結にできる。
// M / 2 <= N < M の場合、連結成分は floor(M / 2) 個である。
// コスト 1 の辺 (M - N + 1, N), ..., (floor(M / 2), M + 1 - floor(M / 2)) をつないだ後、残る連結成分は 1, 2, ..., M - N の M-N 個である。
// このようになったら 1 と他全てを繋ぐのが最善なので、必要なコストの合計は floor(M / 2) - (M - N) + (M - N + 1)(M - N + 2) / 2 - 3 である。
// Tags: minimum-spanning-trees, math
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: i64 = get();
        let m: i64 = get();
        if n >= m {
            println!("{}", m / 2);
            continue;
        }
        println!("{}", (m / 2) - (m - n) + (m - n + 1) * (m - n + 2) / 2 - 3);
    }
}
