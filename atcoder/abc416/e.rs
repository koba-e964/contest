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

const INF: i64 = 1 << 60;

fn wf_once(dist: &mut [Vec<i64>], k: usize, tot: &mut i64) {
    let n = dist.len();
    for i in 0..n {
        for j in 0..n {
            if dist[i][k] < INF && dist[k][j] < INF {
                let new_dist = dist[i][k] + dist[k][j];
                let old_dist = dist[i][j];
                if new_dist < old_dist {
                    dist[i][j] = new_dist;
                    if i < n - 1 && j < n - 1 {
                        *tot += new_dist;
                        if old_dist < INF {
                            *tot -= old_dist;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let n: usize = get();
    let m: usize = get();
    let mut dist = vec![vec![INF; n + 1]; n + 1];
    for i in 0..n + 1 {
        dist[i][i] = 0;
    }
    for _ in 0..m {
        let a: usize = get();
        let b: usize = get();
        let c: i64 = get();
        if dist[a - 1][b - 1] > c * 2 {
            dist[a - 1][b - 1] = c * 2;
            dist[b - 1][a - 1] = c * 2;
        }
    }
    let k: usize = get();
    let t: i64 = get();
    for _ in 0..k {
        let a: usize = get();
        dist[a - 1][n] = t;
        dist[n][a - 1] = t;
    }
    for i in 0..n + 1 {
        wf_once(&mut dist, i, &mut 0);
    }
    let mut tot = 0;
    for i in 0..n {
        for j in 0..n {
            if dist[i][j] < INF {
                tot += dist[i][j];
            }
        }
    }
    for i in 0..n + 1 {
        eprintln!("{i}: {:?}", dist[i]);
    }
    let q: i32 = get();
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let x = get::<usize>() - 1;
            let y = get::<usize>() - 1;
            let c: i64 = get();
            let c = c * 2;
            let old = dist[x][y];
            assert_eq!(dist[y][x], old);
            dist[x][y] = c.min(old);
            dist[y][x] = c.min(old);
            tot += 2 * c.min(old);
            if old < INF {
                tot -= 2 * old;
            }
            wf_once(&mut dist, x, &mut tot);
            wf_once(&mut dist, y, &mut tot);
        } else if ty == 2 {
            let x = get::<usize>() - 1;
            if dist[x][n] > t {
                dist[x][n] = t;
                dist[n][x] = t;
                wf_once(&mut dist, x, &mut tot);
                wf_once(&mut dist, n, &mut tot);
            }
        } else {
            println!("{}", tot / 2);
        }
    }
}
