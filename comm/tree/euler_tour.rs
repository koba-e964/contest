fn euler_tour(v: usize, par: usize, g: &[Vec<usize>],
              rng: &mut [(usize, usize)], cnt: &mut usize) {
    let me = *cnt;
    *cnt += 1;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        euler_tour(w, v, g, rng, cnt);
    }
    rng[v] = (me, *cnt);
}
