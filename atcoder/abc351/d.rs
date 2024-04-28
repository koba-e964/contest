// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Union-Find tree.
// Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

fn main() {
    input! {
        h: usize, w: usize,
        s: [chars; h],
    }
    let mut ma = 1;
    let mut uf = UnionFind::new(5 * h * w);
    let dxy = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
    for x in 0..h {
        for y in 0..w {
            if s[x][y] == '#' { continue; }
            let mut has = false;
            for &(dx, dy) in &dxy {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx >= h || ny >= w { continue; }
                if s[nx][ny] == '#' {
                    has = true;
                }
            }
            if has { continue; }
            let v = x * w + y;
            for dir in 0..4 {
                let (dx, dy) = dxy[dir];
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx >= h || ny >= w { continue; }
                uf.unite(5 * v + dir, 5 * (nx * w + ny) + ((dir + 2) % 4));
                uf.unite(5 * v + 4, 5 * v + dir);
            }
        }
    }
    let mut size = vec![vec![]; 5 * h * w];
    for i in 0..5 * h * w {
        size[uf.root(i)].push(i / 5);
    }
    for i in 0..5 * h * w {
        size[i].sort_unstable();
        size[i].dedup();
        ma = ma.max(size[i].len());
    }
    println!("{}", ma);
}
