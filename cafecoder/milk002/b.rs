fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s: Vec<_> = getline().trim().chars().collect();
    let mut ans = vec![];
    let mut c = 0;
    while let Some(k) = s.pop() {
        if c == 3 {
            c = 0;
            ans.push(',');
        }
        ans.push(k);
        c += 1;
    }
    ans.reverse();
    println!("{}", ans.into_iter().collect::<String>());
}
