fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let c = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let m = c.len();
    let mut d = vec![0; m];
    for _ in 0..ints[0] {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        d[ints[0] - 1] += ints[1] as i32;
    }
    let mut ans = 0;
    for i in 0..m {
        ans += c[i].min(d[i]);
    }
    println!("{ans}");
}
