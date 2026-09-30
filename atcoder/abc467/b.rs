fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<i32>().unwrap();
    let mut ans = 0;
    for _ in 0..n {
        let vals = getline().trim().split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        if vals[2] == "keep" {
            let x = vals[0].parse::<i64>().unwrap();
            let y = vals[1].parse::<i64>().unwrap();
            ans += y - x;
        }
    }
    println!("{ans}");
}
