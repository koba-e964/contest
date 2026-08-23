const INF: i32 = 1 << 28;
let mut dist = vec![vec![INF; w]; h];
let mut que = VecDeque::new();
// FIXME: starting points
for i in 0..h {
    for j in 0..w {
        que.push_back((0, i, j));
    }
}
let dxy = [(0i32, -1i32), (1, 0), (0, 1), (-1, 0)];
while let Some((d, x, y)) = que.pop_front() {
    if dist[x][y] <= d {
        continue;
    }
    dist[x][y] = d;
    for &(dx, dy) in &dxy {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        if nx >= h || ny >= w {
            continue;
        }
        // FIXME: when you can move
        if s[nx][ny] == '.' {
            que.push_back((d + 1, nx, ny));
        }
    }
}
