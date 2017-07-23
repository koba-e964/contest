#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

// http://www.prefield.com/algorithm/basic/template.html
#define FOR(i,c) for(__typeof((c).begin())i=(c).begin();i!=(c).end();++i)
#define ALL(c) (c).begin(), (c).end()


// http://www.prefield.com/algorithm/graph/graph.html
typedef int Weight;
struct Edge {
  int src, dst;
  Edge(int src, int dst) :
    src(src), dst(dst) { }
};
bool operator < (const Edge &e, const Edge &f) {
  return
    e.src != f.src ? e.src < f.src : e.dst < f.dst;
}
typedef vector<Edge> Edges;
typedef vector<Edges> Graph;

typedef vector<Weight> Array;
typedef vector<Array> Matrix;
// http://www.prefield.com/algorithm/graph/bridge.html




void visit(const Graph & g, int v, int u,
    Edges& brdg, vector< vector<int> >& tecomp,
    stack<int>& roots, stack<int>& S, vector<bool>& inS,
    vector<int>& num, int& time) {
  num[v] = ++time;
  S.push(v); inS[v] = true;
  roots.push(v);
  FOR(e, g[v]) {
    int w = e->dst;
    if (num[w] == 0)
      visit(g, w, v, brdg, tecomp, roots, S, inS, num, time);
    else if (u != w && inS[w])
      while (num[roots.top()] > num[w]) roots.pop();
  }
  if (v == roots.top()) {
    brdg.push_back(Edge(u, v));
    tecomp.push_back(vector<int>());
    while (1) {
      int w = S.top(); S.pop(); inS[w] = false;
      tecomp.back().push_back(w);
      if (v == w) break;
    }
    roots.pop();
  }
}
void bridge(const Graph& g, Edges& brdg, vector< vector<int> >& tecomp) {
  const int n = g.size();
  vector<int> num(n);
  vector<bool> inS(n);
  stack<int> roots, S;
  int time = 0;
  REP(u, 0, n) if (num[u] == 0) {
    visit(g, u, n, brdg, tecomp, roots, S, inS, num, time);
    brdg.pop_back();
  }
}


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


/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Header requirement: vector, algorithm
 * Verified by AtCoder ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 */
template<class I, class BiOp = I (*) (I, I)>
class SegTree {
  int n;
  std::vector<I> dat;
  BiOp op;
  I e;
public:
  SegTree(int n_, BiOp op, I e) : op(op), e(e) {
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    dat.resize(2 * n);
    for (int i = 0; i < 2 * n - 1; i++) {
      dat[i] = e;
    }
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    k += n - 1;
    dat[k] = v;
    while (k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
    }
  }
  void update_array(int k, int len, const I *vals) {
    for (int i = 0; i < len; ++i) {
      update(k + i, vals[i]);
    }
  }
  /*
    Updates all elements. O(n)
   */
  void update_all(const I *vals, int len) {
    for (int k = 0; k < std::min(n, len); ++k) {
      dat[k + n - 1] = vals[k];
    }
    for (int k = std::min(n, len); k < n; ++k) {
      dat[k + n - 1] = e;
    }
    for (int b = n / 2; b >= 1; b /= 2) {
      for (int k = 0; k < b; ++k) {
	dat[k + b - 1] = op(dat[k * 2 + b * 2 - 1], dat[k * 2 + b * 2]);
      }
    }
  }
  /* l,r are for simplicity */
  I querySub(int a, int b, int k, int l, int r) const {
    // [a,b) and  [l,r) intersects?
    if (r <= a || b <= l) return e;
    if (a <= l && r <= b) return dat[k];
    I vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2);
    I vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1, 0, 0, n);
  }
};


struct pmin {
  PI operator()(PI a, PI b) const {
    return max(a, b);
  }
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, q;
  cin >> n >> m >> q;
  Graph g(n);
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(Edge(a, b));
    g[b].push_back(Edge(b, a));
  }
  Edges brdg;
  vector< vector<int> > tecomp;
  bridge(g, brdg, tecomp);
  if (0) {
    cerr << "tecomp:\n";
    REP(i, 0, tecomp.size()) {
      cerr << "[" << i << "]";
      REP(j, 0, tecomp[i].size()) {
	cerr << " " << tecomp[i][j];
      }
      cerr << "\n";
    }
  }
  vector<int> comp_map(n);
  int bcc = tecomp.size();
  REP(i, 0, tecomp.size()) {
    REP(j, 0, tecomp[i].size()) {
      comp_map[tecomp[i][j]] = i;
    }
  }
  // Create HL decomposition for the tree tecomp
  HLDecomposition hld(bcc);
  set<PI> added;
  REP(i, 0, n) {
    REP(j, 0, g[i].size()) {
      int w = g[i][j].dst;
      int u = comp_map[i];
      int v = comp_map[w];
      if (u == v) {
	continue;
      }
      if (u > v) {
	swap(u, v);
      }
      if (added.count(PI(u, v)) == 0) {
	hld.add(u, v);
	added.insert(PI(u, v));
      }
    }
  }
  hld.build();
  // Indices of st and ques are mapped on a single interval.
  SegTree<PI, pmin> st(bcc, pmin(), PI(-1, -1));
  vector<priority_queue<int, vector<int> > > ques(bcc);
  REP(loop_cnt, 0, q) {
    int ty, s, t;
    cin >> ty >> s >> t;
    if (ty == 1) { // appear
      s--;
      int u = comp_map[s];
      hld.for_each(u, u, [&](int l, int r) {
	  ques[l].push(t);
	  st.update(l, PI(ques[l].top(), l));
	});
    } else { // check
      s--, t--;
      int u = comp_map[s];
      int v = comp_map[t];
      PI ans(-1, -1);
      hld.for_each(u, v, [&](int l, int r) {
	  ans = max(ans, st.query(l, r));
	});
      if (ans.first < 0) {
	cout << "-1\n";
      } else {
	cout << ans.first << "\n";
	int w = ans.second;
	assert (ques[w].size() > 0);
	ques[w].pop();
	int newtop = ques[w].empty() ? -1 : ques[w].top();
	st.update(w, PI(newtop, w));
      }
    }
    if (0) {
      REP(i, 0, bcc) {
	cerr << "|que[" << i << "]|=" << ques[i].size() << "\n";
      }
    }
  }
}
