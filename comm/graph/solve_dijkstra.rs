// This function returns a Vec consisting of the distances from vertex source.
// This function is especially useful under low-memory constraints
// where storing all edges is not feasible.
// Verified by: https://atcoder.jp/contests/joi2017yo/submissions/24349845
fn solve_dijkstra<F: Fn(usize) -> Vec<(usize, i32)>>(
    n: usize, source: usize, inf: i32, f: F) -> Vec<i32> {
    let mut d = vec![inf; n];
    // que holds (-distance, vertex), so that que.pop() returns the nearest element.
    let mut que = std::collections::BinaryHeap::new();
    que.push((0, source));
    while let Some((cost, pos)) = que.pop() {
        let cost = -cost;
        if d[pos] <= cost {
            continue;
        }
        d[pos] = cost;
        for &(w, c) in &f(pos) {
            let w = w as usize;
            let newcost = cost + c;
            if d[w] > newcost {
                d[w] = newcost + 1;
                que.push((-newcost, w));
            }
        }
    }
    return d;
}
