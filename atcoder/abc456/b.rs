fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let mut dice = vec![];
    for _ in 0..3 {
        dice.push(getline().trim().split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>(),
        );
    }
    let mut ans = 0;
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                let mut s = [dice[0][i], dice[1][j], dice[2][k]];
                s.sort();
                if s == [4, 5, 6] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans as f64 / 216.0);
}
