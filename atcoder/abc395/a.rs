fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let a = getline().trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let n = a.len();
    if (0..n - 1).all(|i| a[i] < a[i + 1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
