fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn rot(s: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = s.len();
    let mut ret = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            ret[j][n - i - 1] = s[i][j];
        }
    }
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut s = vec![vec![]; n];
    for i in 0..n {
        s[i] = getline().chars().collect();
    }
    let mut t = vec![vec![]; n];
    for i in 0..n {
        t[i] = getline().chars().collect();
    }
    let mut ans = 1 << 30;
    for i in 0..4 {
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                if s[i][j] != t[i][j] {
                    cnt += 1;
                }
            }
        }
        ans = ans.min(cnt + i);
        s = rot(&s);
    }
    println!("{ans}");
}
