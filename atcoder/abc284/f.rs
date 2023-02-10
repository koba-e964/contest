fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

// Z algorithm. Calculates an array a[i] = |lcp(s, &s[i..])|,
// where s is the given slice.
// If n = s.length(), the returned array has length n + 1.
// E.g. z_algorithm(b"ababa") = vec![5, 0, 3, 0, 1, 0]
// Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
// Verified by: AtCoder ARC055-C (http://arc055.contest.atcoder.jp/submissions/1061771)
fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n + 1];
    ret[0] = n;
    let mut i = 1; let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] { j += 1; }
        ret[i] = j;
        if j == 0 { i += 1; continue; }
        let mut k = 1;
        while i + k < n && k + ret[k] < j {
            ret[i + k] = ret[k];
            k += 1;
        }
        i += k; j -= k;
    }
    ret
}

fn main() {
    getline();
    let t: Vec<_> = getline().trim().bytes().collect();
    let n = t.len() / 2;
    let mut revtt = t.clone();
    revtt.reverse();
    revtt.extend_from_slice(&t);
    let mut trevt = t.clone();
    trevt.extend_from_slice(&revtt[..2 * n]);
    let z1 = z_algorithm(&trevt);
    let z2 = z_algorithm(&revtt);
    for i in 0..n + 1 {
        if z1[3 * n - i] >= i && z2[2 * n + i] >= n - i {
            println!("{}\n{}", String::from_utf8(revtt[n - i..2 * n - i].to_vec()).unwrap(), i);
            return;
        }
    }
    println!("-1");
}
