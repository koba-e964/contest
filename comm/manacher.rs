// Manacher http://snuke.hatenablog.com/entry/2014/12/02/235837
// Verified by https://atcoder.jp/contests/wupc2019/submissions/4540033
fn manacher<T: PartialEq>(tmp: &[T]) -> Vec<usize> {
    let n = tmp.len();
    let mut r = vec![0; n];
    {
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while i >= j && i + j < n && tmp[i - j] == tmp[i + j] {
                j += 1;
            }
            r[i] = j;
            let mut k = 1;
            while i >= k && i + k < n && k + r[i - k] < j {
                r[i + k] = r[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }
    }
    r
}
