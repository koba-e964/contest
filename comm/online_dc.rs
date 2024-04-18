// Online monotone minima dp. For example, monge dp can be efficiently computed
// by online_dc.
// Verified by: https://yukicoder.me/problems/no/705
// submission: https://yukicoder.me/submissions/566775
const INF: i64 = 1 << 60;

// Complexity: O(n log m + m) where n = r - l, m = b - a.
fn monotone_minima<F>(l: usize, r: usize, a: usize, b: usize,
                      frm: &[i64], lat: &mut [i64],
                      cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    let n = r - l;
    let m = b - a;
    if n == 0 || m == 0 {
        return;
    }
    let mid = (a + b) / 2;
    let mut mi = (INF, n);
    for i in l..r {
        let cost = cost_fun(i, mid);
        mi = std::cmp::min(mi, (cost + frm[i], i));
    }
    let idx = mi.1;
    assert!(l <= idx && idx < r);
    lat[mid] = std::cmp::min(lat[mid], mi.0);
    monotone_minima(l, idx + 1, a, mid, frm, lat, cost_fun);
    monotone_minima(idx, r, mid + 1, b, frm, lat, cost_fun);
}

// Takes O(n log n)-time where n = r - l.
fn induce<F>(l: usize, mid: usize, r: usize, dp: &mut [i64],
             cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    let (frm, lat) = dp.split_at_mut(mid);
    let frm = &frm[l..];
    let lat = &mut lat[..r - mid];
    let inner_cost_fun = |i: usize, j: usize| cost_fun(i + l, j + mid);
    monotone_minima(0, mid - l, 0, r - mid,
                    frm, lat, &inner_cost_fun);
}

// Performs online dp with divide and conquer.
// Converted from the following off-line dp:
// dp[i + 1][j] <--min-- dp[i][k] + cost_fun(k, j)  (k < j)
// Complexity: O(n log^2 n) where n = r - l
fn online_dc<F>(l: usize, r: usize, dp: &mut [i64],
                cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    if l + 1 >= r {
        return;
    }
    let mid = (l + r) / 2;
    online_dc(l, mid, dp, cost_fun);
    induce(l, mid, r, dp, cost_fun);
    online_dc(mid, r, dp, cost_fun);
}
