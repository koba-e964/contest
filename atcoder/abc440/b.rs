fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut t = vec![];
    for i in 0..n {
        t.push((a[i], i + 1));
    }
    t.sort();
    println!("{} {} {}", t[0].1, t[1].1, t[2].1);
}
