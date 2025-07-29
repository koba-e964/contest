fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ints: Vec<usize> = getline().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = ints[0];
    let m = ints[1];
    let mut acc = vec![0; n + 1];
    for _ in 0..m {
        let ints: Vec<usize> = getline().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let l = ints[0];
        let r = ints[1];
        acc[l - 1] += 1;
        acc[r] -= 1;
    }
    for i in 1..=n {
        acc[i] += acc[i - 1];
    }
    println!("{}", acc[..n].iter().min().unwrap());
}
