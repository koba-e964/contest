fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_xor_0(init: usize, k: usize) -> Vec<usize> {
    let rep = if init == 2 {
        4
    } else {
        2
    };
    let mut ret = vec![rep; k];
    ret[0] = init;
    ret[k - 1] = if k % 2 == 0 {
        init
    } else {
        init ^ rep
    };
    ret
}

fn calc(n: usize, k: usize, x: usize, y: usize) -> Option<Vec<usize>> {
    if k == 1 {
        return if x + n - 1 <= y {
            Some((x..x + n - 1).chain(std::iter::once(y)).collect())
        } else {
            None
        };
    }
    if y == 0 { return None; }
    if x != 0 {
        let r = get_xor_0(x, k);
        return Some(r.into_iter().cycle().take(n - 1)
            .chain(std::iter::once(y))
            .collect::<Vec<_>>());
    }
    if n == k + 1 {
        let mut a = vec![0; n];
        a[n - 1] = y;
        return Some(a);
    }
    if n == k + 2 {
        if y == 1 { return None; }
        let mut a = vec![0; n];
        a[n - 2] = 1;
        a[n - 1] = y;
        return Some(a);
    }
    if k == 3 {
        let mut a = vec![0; n];
        a[1] = 1;
        a[2] = 1;
        a[3] = 2;
        a[4] = 3;
        for i in 5..n - 1 {
            a[i] = a[i - 3];
        }
        a[n - 1] = y;
        return Some(a);
    }
    let init = if k >= 4 { get_xor_0(2, k - 2) } else { vec![] };
    let mut a = vec![0; n];
    for i in 0..k - 2 {
        a[i + 1] = init[i];
    }
    a[k - 1] = 2;
    a[k] = 3;
    a[k + 1] = a[1] + 1;
    for i in k + 2..n - 1 {
        a[i] = a[i - k];
    }
    a[n - 1] = y;
    Some(a)
}

// https://yukicoder.me/problems/no/3223 (3.5)
// 構築問題。
// K = 1 の時は X + N - 1 <= Y であることと可能性が同値。以降 K >= 2 とする。
// Y = 0 だと不可能。Y != 0 で X != 0 であれば、 i = 1, ..., N - 1 で A を K 周期にして K 個の XOR を 0 にすることで構築できる。以降 X = 0 かつ Y != 0 とする。
// 0 A[2], ..., A[K], A[K] + 1 A[2] + 1 となるように A[2], A[K] を偶数にしながら A[2..=K] の xor を A[K] にする。A[3..=K+2] は xor が 0 なのでここから周期になるようにする。これは K + 3 <= N かつ K != 3 であれば可能。
// K + 1 = N の場合は A[2], ..., A[K] の xor が 0 になるように調節する。A[2] = ... = A[K] = 0 でよい。
// K + 2 = N の場合は B = A[2] xor ... xor A[K] とおくと A[K+1] > B で A[N] > A[K+1] xor B である。A[K+1] xor B は確実に 1 以上なので Y = 1 の場合は不可能。Y >= 2 であれば可能。
// K = 3 のとき 0 1 1 2 3 で始めれば最後の K 個の xor が 0 である。 これは N >= 6 = K+3 であれば対応可能である。
fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = ints[0];
    let k = ints[1];
    let x = ints[2];
    let y = ints[3];
    if let Some(a) = calc(n, k, x, y) {
        println!("Yes");
        println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    } else {
        println!("No");
    }
}
