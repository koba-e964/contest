fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        if i % 2 == 0 {
            ans += a[i];
        }
    }
    println!("{ans}");
}
