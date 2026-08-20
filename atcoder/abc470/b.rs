fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let c = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();
    let mut f = vec![0; n];
    for c in c {
        f[c] += 1;
    }
    println!("{}", n - f.iter().max().unwrap());
}
