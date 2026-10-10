fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut got = vec![vec![]; n];
    for i in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        for &to in &ints[1..] {
            got[to - 1].push(i + 1);
        }
    }
    for v in got {
        print!("{}", v.len());
        for x in v {
            print!(" {x}");
        }
        println!();
    }
}
