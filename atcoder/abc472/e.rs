fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn dfs(v: usize, c: i8, g: &[Vec<usize>], vis: &mut [bool], col: &mut [i8]) -> Result<(), Vec<usize>> {
    if vis[v] {
        if col[v] == c {
            return Ok(());
        }
        let ret = vec![v];
        return Err(ret);
    }
    vis[v] = true;
    col[v] = c;
    for &w in &g[v] {
        let sub = dfs(w, 1 - c, g, vis, col);
        if let Err(mut ret) = sub {
            if ret.len() >= 2 && ret[0] == ret[ret.len() - 1] {
                return Err(ret);
            }
            ret.push(v);
            return Err(ret);
        }
    }
    Ok(())
}

fn main() {
    let t = getline().trim().parse::<i32>().unwrap();
    'outer:
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [n, m] = ints[..] else { panic!() };
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let ints = getline().trim().split_whitespace()
                .map(|x| x.parse::<usize>().unwrap() - 1)
                .collect::<Vec<_>>();
            let [a, b] = ints[..] else { panic!() };
            g[a].push(b);
            g[b].push(a);
        }
        let mut vis = vec![false; n];
        let mut col = vec![0; n];
        for i in 0..n {
            if vis[i] { continue; }
            let res = dfs(i, 0, &g, &mut vis, &mut col);
            if let Err(mut v) = res {
                v.pop();
                for v in &mut v {
                    *v += 1;
                }
                println!("{}", v.len());
                for v in v {
                    print!("{v} ");
                }
                println!();
                continue 'outer;
            }
        }
        println!("-1");
    }
}
