/// Coordinate compression
/// Returns a vector of usize, with i-th element the "rank" of a[i] in a.
/// The property forall i. inv_map[ret[i]] == a[i] holds.
/// Verified by: yukicoder No.404 (http://yukicoder.me/submissions/155377)
fn coord_compress<T: Ord>(a: &[T])
                          -> (Vec<usize>, Vec<&T>) {
    let n = a.len();
    let mut cp: Vec<(&T, usize)> = (0 .. n).map(|i| (&a[i], i)).collect();
    cp.sort();
    let mut inv_map = Vec::with_capacity(n);
    let mut prev: Option<&T> = None;
    let mut ret = vec![0; n];
    let mut cnt = 0;
    for (v, i) in cp {
        if prev == Some(v) {
            ret[i] = cnt - 1;
            continue;
        }
        ret[i] = cnt;
        inv_map.push(v);
        prev = Some(v);
        cnt += 1;
    }
    (ret, inv_map)
}
