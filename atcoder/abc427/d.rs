fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let t = getline().trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [n, m, k] = ints[..] else { panic!() };
        let s = getline().trim().chars().collect::<Vec<_>>();
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let ints = getline().trim().split_whitespace()
                .map(|x| x.parse::<usize>().unwrap() - 1)
                .collect::<Vec<_>>();
            let [u, v] = ints[..] else { panic!() };
            g[u].push(v);
        }
        let mut dp = vec![false; n];
        for i in 0..n {
            dp[i] = s[i] == 'A';
        }
        for _ in 0..2 * k {
            let mut ep = vec![false; n];
            for i in 0..n {
                let mut win = false;
                for &w in &g[i] {
                    if !dp[w] { win = true; }
                }
                ep[i] = win;
            }
            dp = ep;
        }
        println!("{}", if dp[0] {
            "Alice"
        } else {
            "Bob"
        });
    }
}
