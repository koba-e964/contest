fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn min(x: Option<String>, b: String) -> Option<String> {
    match x {
        Some(a) => Some(if a.len() < b.len() { a } else { b }),
        None => Some(b),
    }
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut ex: Vec<Option<String>> = vec![None; n + 1];
    let mut te: Vec<Option<String>> = vec![None; n + 1];
    let mut fa: Vec<Option<String>> = vec![None; n + 1];
    let mut nu: Vec<Option<String>> = vec![None; n + 1];
    {
        let mut c = 1;
        let mut l = 1;
        while c <= n {
            nu[c] = Some("1".repeat(l));
            c = 10 * c + 1;
            l += 1;
        }
    }
    // dp[i] := i を作るための最短の式 とする (dp= ex, te, fa, nu)
    // ダイクストラ法のように、 (i, 式の長さ, 式を構成する規則の個数) の小さい順 (辞書順) で埋めていく
    // factor -> term -> expr -> factor -> term の順に遷移すれば全ての必要な遷移が埋め尽くされる。
    for i in 0..n + 1 {
        // factor
        fa[i] = nu[i].clone();

        // term
        let mut me = fa[i].clone();
        for j in 2..i {
            if i % j != 0 {
                continue;
            }
            if let Some(ref v) = te[i / j] {
                if let Some(ref w) = fa[j] {
                    let mut v = v.clone();
                    v.push('*');
                    v.push_str(w);
                    me = min(me, v);
                }
            }
        }
        te[i] = me;

        // expr
        me = te[i].clone();
        for j in 1..i {
            if let Some(ref v) = ex[j] {
                if let Some(ref w) = te[i - j] {
                    let mut v = v.clone();
                    v.push('+');
                    v.push_str(w);
                    me = min(me, v);
                }
            }
        }
        ex[i] = me;

        // factor (refrain)
        let mut me = fa[i].clone();
        if let Some(mut v) = ex[i].clone() {
            v.insert(0, '(');
            v.push(')');
            me = min(me, v);
        }
        fa[i] = me;

        // term (refrain)
        let mut me = fa[i].clone();
        if let Some(mut v) = te[i].clone() {
            me = min(me, v);
        }
        te[i] = me;
    }
    println!("{}", ex[n].clone().unwrap());
}
