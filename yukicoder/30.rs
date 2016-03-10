use std::cmp::*;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn parse<T : std::str::FromStr>(s : &str) -> T {
    match s.parse::<T>() {
        Ok(t) => t,
        _    => panic!(),
    }
}

fn rec(n: usize, memo: &mut Vec<Vec<i32>>, dep: &mut Vec<Vec<(usize, i32)>>, idx: usize) {
    if memo[idx][0] >= 0 {
        return;
    }
    if dep[idx].len() == 0 {
        for i in 0 .. n {
            memo[idx][i] = 0;
        }
        memo[idx][idx] = 1;
        return;
    }
    for j in 0 .. n {
        memo[idx][j] = 0;
    }
    for i in 0 .. dep[idx].len() {
        let (to, size) = dep[idx][i];
        rec(n, memo, dep, to);
        for j in 0 .. n {
            memo[idx][j] += memo[to][j] * size;
        }
    }
}

fn main() {
    let n: usize = parse(getline().trim());
    let m: usize = parse(getline().trim());
    let mut pqrs: Vec<(usize, i32, usize)> = vec![(0,0,0); m];
    let mut dep: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
    let mut memo: Vec<Vec<i32> > = vec![Vec::new(); n];
    for i in 0 .. m {
        let s = getline().trim().split(" ").map(|x| parse::<usize>(x)).collect::<Vec<usize>>();
        pqrs[i] = (s[0] - 1, s[1] as i32, s[2] - 1);
        dep[s[2] - 1].push((s[0] - 1, s[1] as i32));
    }
    for i in 0 .. n {
        memo[i] = vec![0; n];
        memo[i][0] = -1;
    }
    rec(n, &mut memo, &mut dep, n - 1);
    for i in 0 .. (n - 1) {
        println!("{}", memo[n - 1][i]);
    }
}
