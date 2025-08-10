fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn sub(a: i64, (b0, b1): (i64, i64)) -> (i64, i64) {
    (a - b0, -b1).max((0, 0))
}

fn calc_cycle5(a: &[i64], r: i64) -> (i64, i64) {
    let x3 = sub(a[3], (r, 1));
    let x0 = sub(a[4], (r, 1));
    let x1 = sub(a[0], x0);
    let x2 = sub(a[1], x1);
    let x2 = x2.max(sub(a[2], x3));
    (x0.0 + x1.0 + x2.0 + x3.0 + r, x0.1 + x1.1 + x2.1 + x3.1)
}

fn opt(a: &[i64]) -> i64 {
    let mut pass = a[4];
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let (_, g) = calc_cycle5(a, mid);
        if g >= 0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    calc_cycle5(a, pass).0
}

// https://yukicoder.me/problems/no/1122 (4)
// https://chatgpt.com/share/68982fcf-b918-8010-bb2f-45545a394a56 によれば有理数の最適解は分母が 1,2,3 のいずれかである。
// The author read the editorial before implementing this.
// 二分探索で飛行機に乗る回数が x 回にできるか、という問題に変換すると、
// 補集合を取ることで以下のような問題に変換できる:
// a + b >= x - A, b + c >= x - B, ..., a+b+c+d+e <= x を満たす整数 a,b,c,d,e は存在するか?
// これを a+b+c+d+e を最小化する問題に変換し、e を固定したときの最小値を I(e) と置くと、I(e) は下に凸である:
// 証明: e1 と e2 における最適値 I(e1), I(e2) を実現する解をそれぞれ S1, S2 とする。
// 0 <= u <= 1 とする。e = u e1 + (1-u) e2 に対する解として u S1 + (1-u) S2 は実行可能でスコアが u I(e1) + (1-u) I(e2) であり、
// 最適値 I(e) はこれ以下である。
// よって三分探索か、微分係数を調べるタイプの二分探索で正解できる。
// これは大きさ 5 のサイクルグラフの辺被覆のような問題を、一点固定して「切り開く」とみなせる。
// Tags: integer-linear-programming, integer-programming
fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut pass = 0;
    let mut fail = 1 << 53;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        let comp = ints.iter().map(|&x| 0.max(mid - x)).collect::<Vec<_>>();
        if opt(&comp) <= mid {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{pass}");
}
