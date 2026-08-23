fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// - bl = 0: n -> 0 でループ。oだったら隣に移譲、xだったらそこでやめ
// - bl >= 1: bl-1 の結果を使う
// - k = 1..=n でループ。立っているビットごとに進める
fn main() {
    getline();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    const B: usize = 20;
    let mut bin = vec![vec![0; n + 1]; B];
    bin[0][n] = n;
    for i in (0..n).rev() {
        if s[i] == 'o' {
            bin[0][i] = bin[0][i + 1];
        } else {
            bin[0][i] = i + 1;
        }
    }
    for i in 0..B - 1 {
        for j in 0..=n {
            bin[i + 1][j] = bin[i][bin[i][j]];
        }
    }
    for k in 1..=n {
        let mut x = 0;
        for i in 0..B {
            if (k & 1 << i) != 0 {
                x = bin[i][x];
            }
        }
        println!("{x}");
    }
}
