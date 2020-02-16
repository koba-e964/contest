// Ref: https://codeforces.com/contest/1200/submission/58594933
// Verified by: https://codeforces.com/contest/1200/submission/58692231
fn kmp_ff<T: PartialEq>(pat: &[T]) -> Vec<usize> {
    let n = pat.len();
    let mut pi = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        while j > 0 && pat[i] != pat[j] {
            j = pi[j - 1];
        }
        j += usize::from(pat[i] == pat[j]);
        pi[i] = j;
    }
    pi
}
