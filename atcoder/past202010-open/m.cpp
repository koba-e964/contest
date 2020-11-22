#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;


/// http://pekempey.hatenablog.com/entry/2015/11/06/193123

//*****************************************************************
//	HL Decomposition のライブラリ
//*****************************************************************
struct HLDecomposition {
	vector<vector<int> > g;

	// vid, head, heavy, parent は必須
	// depth, inv は使用する機能によっては不要
	vector<int> vid, head, heavy, parent, depth, inv;

	HLDecomposition(int n) : g(n), vid(n, -1), head(n), heavy(n, -1), parent(n), depth(n), inv(n) {}

	// 辺 (u, v) を追加する
	void add(int u, int v) {
		g[u].push_back(v);
		g[v].push_back(u);
	}

	// 構築する
	void build() {
		dfs(0, -1);
		bfs();
	}

	int dfs(int curr, int prev) {
		parent[curr] = prev;
		int sub = 1, max_sub = 0;
		for (int next : g[curr]) if (next != prev) {
			depth[next] = depth[curr] + 1;
			int sub_next = dfs(next, curr);
			sub += sub_next;
			if (max_sub < sub_next) max_sub = sub_next, heavy[curr] = next;
		}
		return sub;
	}

	void bfs() {
		int k = 0;
		queue<int> q;
		q.push(0);
		while (!q.empty()) {
			int h = q.front(); q.pop();
			for (int i = h; i != -1; i = heavy[i]) {
				vid[i] = k++;
				inv[vid[i]] = i;
				head[i] = h;
				for (int j : g[i]) if (j != parent[i] && j != heavy[i]) q.push(j);
			}
		}
	}

	// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
	//   以下の関数は必要に応じて実装
	// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

	// 頂点属性の for_each
	void for_each(int u, int v, function<void(int, int)> f) {
		if (vid[u] > vid[v]) swap(u, v);
		f(max(vid[head[v]], vid[u]), vid[v]);
		if (head[u] != head[v]) for_each(u, parent[head[v]], f);
	}

	// 頂点属性の for_each （有向
	// fの3番目の引数には順方向なら0、逆方向なら1が渡される
	void for_each_directed(int u, int v, function<void(int, int, int)> f) {
		if (vid[u] > vid[v]) {
			f(max(vid[head[u]], vid[v]), vid[u], 1);
			if (head[u] != head[v]) for_each_directed(parent[head[u]], v, f);
		} else {
			f(max(vid[head[v]], vid[u]), vid[v], 0);
			if (head[u] != head[v]) for_each_directed(u, parent[head[v]], f);
		}
	}

	// 辺属性の for_each
	void for_each_edge(int u, int v, function<void(int, int)> f) {
		if (vid[u] > vid[v]) swap(u, v);
		if (head[u] != head[v]) {
			f(vid[head[v]], vid[v]);
			for_each_edge(u, parent[head[v]], f);
		} else {
			if (u != v) f(vid[u] + 1, vid[v]);
		}
	}

	// 頂点 u の d 個上の頂点を求める（存在しないなら0を返す）
	int ancestor(int u, int d) {
		while (true) {
			if (depth[head[u]] > depth[u] - d) {
				d -= depth[u] - depth[head[u]] + 1;
				if (head[u] == 0) return 0;
				u = parent[head[u]];
			} else {
				return inv[vid[u] - d];
			}
		}
	}

	// 頂点 u と頂点 v の LCA を求める
	int lca(int u, int v) {
		if (vid[u] > vid[v]) swap(u, v);
		if (head[u] == head[v]) return u;
		return lca(u, parent[head[v]]);
	}

	// 頂点 u と頂点 v の距離を求める
	int distance(int u, int v) {
		return depth[u] + depth[v] - 2 * depth[lca(u, v)];
	}
};

// codeforces/edu54/g.cpp
template<class T, class U>
struct LazySegTree {
  int n;
  vector<T> dat;
  vector<U> lazy;
  T e;
  U upe; // upe: identity for upop
  static void biop_assign(T &u, const T &a, const T &b) {
    u = a | b;
  }
  static void upop(T &a, U b) {
    if (b != -1) {
      a = b;
    }
  }
  static U upbiop(U a, U b) {
    return b == -1 ? a : b;
  }
  LazySegTree(int n_, T e, U upe): e(e), upe(upe) {
    n = 1;
    while (n < n_) n *= 2;
    dat = vector<T>(2 * n - 1, e);
    lazy = vector<U>(2 * n - 1, upe);
  }
  void lazy_evaluate_node(int k) {
    upop(dat[k], lazy[k]);
    if (k < n - 1) {
      lazy[2 * k + 1] = upbiop(lazy[2 * k + 1], lazy[k]);
      lazy[2 * k + 2] = upbiop(lazy[2 * k + 2], lazy[k]);
    }
    lazy[k] = upe;
  }
  void update_node(int k) {
    assert (0 <= k && k < n - 1);
    biop_assign(dat[k], dat[2 * k + 1], dat[2 * k + 2]);
  }
  void update_sub(int a, int b, U v, int k, int l, int r) {
    lazy_evaluate_node(k);
    if (r <= a || b <= l) return;
    if (a <= l && r <= b) {
      lazy[k] = upbiop(lazy[k], v);
      lazy_evaluate_node(k);
      return;
    }
    int mid = (l + r) / 2;
    update_sub(a, b, v, 2 * k + 1, l, mid);
    update_sub(a, b, v, 2 * k + 2, mid, r);
    update_node(k);
  }
  // Updates [a, b)
  void update(int a, int b, U v) {
    update_sub(a, b, v, 0, 0, n);
  }
  void update_single(int a, T val, int k, int l, int r) {
    lazy_evaluate_node(k);
    if (r <= a || a + 1 <= l) return;
    if (a <= l && r <= a + 1) {
      dat[k] = val;
      return;
    }
    int mid = (l + r) / 2;
    update_single(a, val, 2 * k + 1, l, mid);
    update_single(a, val, 2 * k + 2, mid, r);
    update_node(k);
  }
  void update_single(int a, T val) {
    update_single(a, val, 0, 0, n);
  }
  int query_sub(int a, int b, int k, int l, int r) {
    lazy_evaluate_node(k);
    if (r <= a || b <= l) return 0;
    if (a <= l && r <= b) return dat[k];
    int mid = (l + r) / 2;
    int vr = query_sub(a, b, 2 * k + 2, mid, r);
    int vl = query_sub(a, b, 2 * k + 1, l, mid);
    update_node(k);
    return vl | vr;
  }
  // [a, b)
  int query(int a, int b) {
    return query_sub(a, b, 0, 0, n);
  }
};

const int N = 110000;
PI par[N];
vector<PI> g[N];

void dfs(int v, int p) {
  par[v] = PI(-1, -1);
  for (PI e: g[v]) {
    int w = e.first, idx = e.second;
    if (w == p) continue;
    dfs(w, v);
    par[w] = PI(v, idx);
  }
}

// Tags: hldecomposition, heavy-light-decomposition, tree, lazy-segment-tree
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  HLDecomposition hld(n);
  vector<PI> edges;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    hld.add(a, b);
    g[a].push_back(PI(b, i));
    g[b].push_back(PI(a, i));
    edges.push_back(PI(a, b));
  }
  hld.build();
  LazySegTree<int, int> lst(n, 0, -1);
  REP(i, 0, q) {
    int u, v, c;
    cin >> u >> v >> c;
    u--, v--;
    hld.for_each_edge(u, v, [&](int l, int r) {
                              if (DEBUG) cerr << "lr " << l << " " << r << endl;
                              lst.update(l, r + 1, c);
                            });
    if (DEBUG) {
      REP(i, 0, n) {
        cerr << " " << lst.query(i, i + 1);
      }
      cerr << endl;
    }
  }
  REP(i, 0, n - 1) {
    int a = edges[i].first, b = edges[i].second;
    int r;
    if (par[a].first == b) {
      r = a;
    } else {
      r = b;
    }
    int ans = 0;
    hld.for_each_edge(r, a + b - r, [&](int l, int r) {
                         if (DEBUG) cerr << "l=r " << l << " " << r << endl;
                         ans |= lst.query(l, l + 1);
                       });
    cout << ans << endl;
  }
}
