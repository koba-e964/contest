fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let fst = getline();
    let fst = fst.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let d = fst[1];
    let s = getline();
    println!("{}", s.chars().filter(|&c| c == '.').count() + d);
}
