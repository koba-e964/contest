fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn dfs(s: &[String], k: usize, acc: &mut String, res: &mut Vec<String>) {
    if k == 0 {
        res.push(acc.clone());
        return;
    }
    for i in 0..s.len() {
        let mut acc = acc.clone();
        acc.push_str(&s[i]);
        dfs(&s, k - 1, &mut acc, res);
    }
}

fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let n = ints[0];
    let k = ints[1];
    let x = ints[2] - 1;
    let mut s = vec![];
    for _ in 0..n {
        s.push(getline().trim().to_string());
    }
    let mut res = vec![];
    dfs(&s, k, &mut "".to_string(), &mut res);
    res.sort_unstable();
    println!("{}", res[x]);
}
