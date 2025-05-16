fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let a = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();
    let mut occ = vec![vec![]; n];
    for i in 0..n {
        occ[a[i]].push(i + 1);
    }
    let nn = n as i64;
    let mut ans = nn * (nn + 1) / 2 * nn;
    for i in 0..n {
        occ[i].insert(0, 0);
        occ[i].push(n + 1);
        for j in 0..occ[i].len() - 1 {
            let l = occ[i][j];
            let r = occ[i][j + 1];
            let len = (r - l - 1) as i64;
            ans -= len * (len + 1) / 2;
        }
    }
    println!("{ans}");
}
