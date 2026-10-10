fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], a: &[i64]) -> (i64, i64) {
    let mut ans = 0;
    let mut ma = 0;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        let (sub, subma) = dfs(w, v, g, a);
        let this = a[v] + a[w];
        if sub + this > 0 {
            ans += sub + this;
        }
        ma = ma.max(subma);
    }
    ma = ma.max(ans);
    (ans, ma)
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();
        let [u, v] = ints[..] else { panic!() };
        g[u].push(v);
        g[v].push(u);
    }
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let (_, ans) = dfs(0, n, &g, &a);
    println!("{ans}");
}
