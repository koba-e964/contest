fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let mut ans = vec![-1; m];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let [c, s] = ints[..] else { panic!() };
        let c = c as usize - 1;
        ans[c] = ans[c].max(s);
    }
    for i in 0..m {
        print!("{}{}", ans[i], if i + 1 == m { "\n" } else { " " });
    }
}
