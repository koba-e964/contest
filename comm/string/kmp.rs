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

// Verified by: https://yukicoder.me/submissions/852992
fn contains<T: PartialEq>(s: &[T], pat: &[T]) -> bool {
    if pat.is_empty() {
        return true;
    }
    let ff = kmp_ff(pat);
    let mut pos = 0;
    for i in 0..s.len() {
        let c = &s[i];
        while pos > 0 && &pat[pos] != c {
            pos = ff[pos - 1];
        }
        if &pat[pos] == c {
            pos += 1;
        }
        if pos == pat.len() {
            return true;
        }
    }
    false
}
