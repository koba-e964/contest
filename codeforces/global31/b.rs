fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let t = getline().trim().parse::<i32>().unwrap();
    for _ in 0..t {
        getline();
        let s = getline().trim().split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let mut ans = String::new();
        for s in s {
            let x = ans.clone() + &s;
            let y = s + &ans;
            ans = std::cmp::min(x, y);
        }
        println!("{ans}");
    }
}
