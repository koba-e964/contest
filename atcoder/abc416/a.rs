fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let s = getline().chars().collect::<Vec<char>>();
    println!("{}", if s[ints[1] - 1..ints[2]].iter().all(|&c| c == 'o') {
        "Yes"
    } else {
        "No"
    });
}
