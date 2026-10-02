fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn calc(wxv: &[(i64, i64, i64)], a: i64) -> i64 {
    let mut ma = 0;
    let inittot;
    // t = 0
    {
        let mut tot = 0;
        for &(w, x, _) in wxv {
            if x >= 0 && x <= a {
                tot += w;
            }
        }
        ma = ma.max(tot);
        inittot = tot;
    }
    {
        let n = wxv.len();
        let mut ev = vec![];
        for i in 0..n {
            let (w, x, v) = wxv[i];
            let (will_cross, time, inout) = if v > 0 {
                (a >= x, (a - x, v), -1)
            } else if v < 0 {
                (a < x, (x - a, -v), 1)
            } else {
                (a == x, (100000, 1), -1)
            };
            if will_cross {
                ev.push((time, w, inout));
            }
        }
        for i in 0..n {
            let (w, x, v) = wxv[i];
            let (will_cross, time, inout) = if v > 0 {
                (0 > x, (-x, v), 1)
            } else if v < 0 {
                (0 <= x, (x, -v), -1)
            } else {
                (0 == x, (100000, 1), -1)
            };
            if will_cross {
                ev.push((time, w, inout));
            }
        }
        ev.sort_by(|&((num1, den1), _, inout1), ((num2, den2), _, inout2)| {
            let t1 = num1 * den2;
            let t2 = num2 * den1;
            t1.cmp(&t2).then(
                inout2.cmp(&inout1)
            )
        });
        if n <= 5 {
            eprintln!("wxv = {wxv:?}, a = {a}");
            eprintln!("ev = {ev:?}");
        }
        let mut cur = inittot;
        for (_, w, inout) in ev {
            cur += w * inout;
            ma = ma.max(cur);
        }
    }
    if wxv.len() <= 5 {
        eprintln!("ma = {ma}\n");
    }
    ma
}

// - 一つの点を原点として、そこから右に長さAの範囲を考える。[0, A] から出たり入ったりするイベントを作り、自国でソートしてmaxを探る。
//   - t = 0 のとき
//   - 他の点が座標 A に差し掛かった瞬間
//   - 他の点が座標 0 に差し掛かった瞬間
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [n, a] = ints[..] else { panic!() };
    let n = n as usize;
    let mut wxv = vec![];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [w, x, v] = ints[..] else { panic!() };
        wxv.push((w, x, v));
    }

    let mut ma = 0;
    for i in 0..n {
        let (w, x, v) = wxv[i];
        let mut rest = vec![];
        for j in 1..n {
            let (wj, xj, vj) = wxv[(j + i) % n];
            rest.push((wj, xj - x, vj - v));
        }
        ma = ma.max(w + calc(&rest, a));
    }
    println!("{ma}");
}
