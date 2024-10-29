fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut b = vec![vec![]; 8];
    for i in 0..8 {
        let s = getline().chars().collect::<Vec<_>>();
        b[i] = s;
    }
    let mut ans = 0;
    for i in 0..8 {
        for j in 0..8 {
            if b[i][j] == '#' {
                for a in 0..8 {
                    if b[a][j] != '#' {
                         b[a][j] = 'x';
                    }
                }
                for a in 0..8 {
                    if b[i][a] != '#' {
                         b[i][a] = 'x';
                    }
                }
            }
        }
    }
    for i in 0..8 {
        ans += b[i].iter().filter(|&&c| c == '.').count();
    }
    println!("{}", ans);
}
