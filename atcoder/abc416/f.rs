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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

const INF: i64 = 1 << 60;

fn dfs(v: usize, par: usize, g: &[Vec<usize>], a: &[i64], k: usize) -> Vec<[i64; 3]> {
    // [how many paths have ended][0: root not taken, 1: root taken but ends here, 2: root taken and continues]
    let mut me = vec![[-INF; 3]; k + 1];
    me[0][0] = 0;
    me[0][2] = a[v];
    for &u in &g[v] {
        if u != par {
            let mut nxt = vec![[-INF; 3]; k + 1];
            let sub = dfs(u, v, g, a, k);
            for i in 0..=k {
                for b in 0..3 {
                    if me[i][b] == -INF {
                        continue;
                    }
                    for j in 0..=k - i {
                        for c in 0..3 {
                            if sub[j][c] == -INF {
                                continue;
                            }
                            if c == 2 && b == 1 {
                                continue;
                            }
                            let mut new = me[i][b] + sub[j][c];
                            let mut nidx = i + j;
                            let mut nidx2 = b;
                            if c == 2 {
                                if b == 0 {
                                    new += a[v];
                                    nidx2 = 2;
                                } else {
                                    nidx += 1;
                                    nidx2 = 1;
                                }
                            }
                            if nidx <= k {
                                nxt[nidx][nidx2] = nxt[nidx][nidx2].max(new);
                            }
                        }
                    }
                }
            }
            me = nxt;
        }
    }
    for i in 0..k {
        me[i + 1][1] =me[i + 1][1].max(me[i][2]);
    }
    me
}

fn solve() {
    let n: usize = get();
    let k: usize = get();
    let a: Vec<i64> = (0..n).map(|_| get::<i64>()).collect();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        g[a].push(b);
        g[b].push(a);
    }
    let res = dfs(0, n, &g, &a, k);
    let mut ans = 0;
    for i in 0..=k {
        ans = ans.max(res[i][0]);
        ans = ans.max(res[i][1]);
        if i < k {
            ans = ans.max(res[i][2]);
        }
    }
    println!("{ans}");
}
