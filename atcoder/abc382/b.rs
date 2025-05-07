fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let fst = getline();
    let fst = fst.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut d = fst[1];
    let mut s = getline().trim().chars().collect::<Vec<_>>();
    for i in (0..s.len()).rev() {
        if d > 0 && s[i] == '@' {
            s[i] = '.';
            d -= 1;
        }
    }
    println!("{}", s.into_iter().collect::<String>());
}
