fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let mut g = vec![vec![]; n];
    let mut vis = vec![false; n];
    let mut que = vec![];
    for i in 0..n {
        let ints = getline().trim().split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
        if ints[0] + ints[1] == 0 {
            vis[i] = true;
            que.push(i);
        }
        for v in ints {
            if v != 0 {
                g[v - 1].push(i);
            }
        }
    }
    while let Some(v) = que.pop() {
        for &w in &g[v] {
            if vis[w] { continue; }
            vis[w] = true;
            que.push(w);
        }
    }
    println!("{}", vis.iter().filter(|&&v| v).count());
}
