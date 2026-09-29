fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let d = ints[1];
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let mut ans = vec![];
    for i in 0..n {
        if (0..n).all(|j| i == j || (a[i] - a[j]).abs() >= d) {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{}{}", ans[i], if i + 1 == ans.len() { "" } else { " " });
    }
    println!();
}
