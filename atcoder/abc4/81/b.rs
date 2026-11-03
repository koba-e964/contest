fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let k = ints[1];
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut ans = i32::from(a[0] == k);
    for i in 1..a.len() {
        if (a[i - 1] - k) * (a[i] - k) < 0 {
            ans += 1;
        }
        if a[i] == k {
            ans += 1;
        }
    }
    println!("{ans}");
}
