#include <cassert>
#include <iomanip>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;

// Copied from https://kokiymgch.hatenablog.com/entry/2018/03/21/174958

//e2i[edge] -> index of the edge
//tree index : [0, 1, ..., bc.size() - 1] -> component
//tree index : [bc.size(), bc.size() + 1, ..., bc.size() + art.size() - 1] -> articulation
//cmp[index of edge] -> index of the node of the constructed tree
//cmp_node[index of node] -> -1 if it's not articulation, otherwise index of the node of the constructed tree
struct BiconnectedComponentTree {
        vector<int> ord, low, art, cmp, cmp_node;
        vector<bool> used;
        vector<vector<int>> g, tree;
        vector<vector<pair<int, int>>> bc;
        vector<pair<int, int>> tmp;
        map<pair<int, int>, int> e2i;
        int n, m, k = 0;
        BiconnectedComponentTree(const vector<vector<int>> &g, const map<pair<int, int>, int> &e2i) : g(g), e2i(e2i) {
                n = g.size();
                m = e2i.size();
                cmp_node.resize(n, -1);
                cmp.resize(m, -1);
                ord.resize(n, -1);
                low.resize(n, -1);
                used.resize(n, false);
        }
        void dfs(int u, int prev) {
                used[u] = true;
                ord[u] = k ++;
                low[u] = ord[u];
                bool is_art = false;
                int cnt = 0;
                for (auto v : g[u]) if (v != prev) {
                        if (ord[v] < ord[u]) { 
                                tmp.emplace_back(min(u, v), max(u, v));
                        }
                        if (!used[v]) {
                                cnt ++;
                                dfs(v, u);
                                low[u] = min(low[u], low[v]);
                                if (prev != -1 && low[v] >= ord[u]) {
                                        is_art = true;
                                }
                                if (low[v] >= ord[u]) {
                                        bc.push_back({});
                                        while (true) {
                                                pair<int, int> e = tmp.back();
                                                bc.back().emplace_back(e);
                                                tmp.pop_back();
                                                if (min(u, v) == e.first && max(u, v) == e.second) {
                                                        break;
                                                }
                                        }
                                }
                        } else {
                                low[u] = min(low[u], ord[v]);
                        }
                }
                if (prev == -1 && cnt > 1) is_art = true;
                if (is_art) art.push_back(u);
        }
        void build() {
                dfs(0, -1);
                for (int i = 0; i < (int) bc.size(); i ++) {
                        for (auto e : bc[i]) {
                                cmp[e2i[make_pair(min(e.first, e.second), max(e.first, e.second))]] = i;
                        }
                }
                tree.resize(bc.size() + art.size());
                for (int i = 0; i < (int) art.size(); i ++) {
                        int j = i + (int) bc.size();
                        cmp_node[art[i]] = j;
                        int u = art[i];
                        set<int> tmp;
                        for (auto v : g[u]) {
                                int t = cmp[e2i[make_pair(min(u, v), max(u, v))]];
                                if (tmp.count(t) == 0) {
                                        tmp.insert(t);
                                }
                        }
                        for (auto v : tmp) {
                                tree[j].push_back(v);
                                tree[v].push_back(j);
                        }
                }
        }
};

/**
 * Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
 * Header Requirement: vector, cassert
 * Verified by: AtCoder ABC014-D (http://abc014.contest.atcoder.jp/submissions/759125)
 */
class LowestCommonAncestor {
private:
  int n, bn;
  std::vector<int> parent; // 0 is root, parent[0] = 0
  std::vector<int> dep;
  
  // Lowest Common Ancestor
  
  std::vector<std::vector<int> > lca_tbl;
  
  void dfs(const std::vector<std::vector<int> > &edges, int v, int par, int d) {
    parent[v] = par;
    dep[v] = d;
    
    for (int i = 0; i < edges[v].size(); ++i) {
      int u = edges[v][i];
      if (u != par) {
	dfs(edges, u, v, d + 1);
      }
    }
  }
  
  void lca_init(void) {
    for (int v = 0; v < n; ++v) {
      lca_tbl[v] = std::vector<int>(bn + 1, 0);
      lca_tbl[v][0] = parent[v];
    }
    for (int i = 1; i <= bn; ++i) {
      for (int v = 0; v < n; ++v) {
	lca_tbl[v][i] = lca_tbl[lca_tbl[v][i - 1]][i - 1];
      }
    }
  }
public:
  int lca(int x, int y) const {
    int dx = dep[x];
    int dy = dep[y];
    if (dx > dy) {
      return lca(y, x);
    }
    // Go up from y to the depth of x
    for (int l = bn; l >= 0; --l) {
      if (dy - dx >= 1 << l) {
	y = lca_tbl[y][l];
	dy -= 1 << l;
      }
    }

    assert (dx == dy);

    if (x == y) {
      return x;
    }
  
    for (int l = bn; l >= 0; --l) {
      if (lca_tbl[x][l] != lca_tbl[y][l]) {
	x = lca_tbl[x][l];
	y = lca_tbl[y][l];
      }
    }
    return lca_tbl[x][0];
  }
  int depth(int a) const {
    return dep[a];
  }
  LowestCommonAncestor(int n, const std::vector<std::vector<int> > &edges)
    : n(n), parent(n), dep(n), lca_tbl(n) {
    bn = 0;
    while (n > 1 << bn) {
      bn++;
    }
    dfs(edges, 0, 0, 0);
    lca_init();
  }
};

void dfs(int v, int par, const vector<VI> &g, int d, VI &dep) {
  dep[v] = d;
  for (int w: g[v]) {
    if (w != par) dfs(w, v, g, d + 1, dep);
  }
}

// https://yukicoder.me/problems/no/1326 (4)
// グラフが木であれば、(x と y の間にある頂点数) = (x と y の距離) - 1 なので簡単。
// 一般のグラフの場合、二重辺連結成分分解を行ってできた木の各頂点について、
// 元のグラフにおける寄与が 0, 1, 2 のいずれかである。
// これは、頂点数 1 の成分については木の頂点をそのままにし、頂点数が 2 以上である成分については
// 木における頂点を通るだけで距離が 1 増えるように、木の頂点を増幅させれば良い。
// それは元々の頂点を v としたとき v に接続する辺ごとに新しく頂点を作りそれぞれの辺と接続させ、
// それらと v を距離 0.5 で繋げばできる。
// Similar problems: https://yukicoder.me/problems/no/1983
// -> 間違い。二重辺連結成分分解ではなく二重頂点連結成分分解をする必要がある。
// これは関節点と結びつく概念である。他人のライブラリを拝借して AC。
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<VI> g(n);
  map<PI, int> e2i;
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    g[u].push_back(v);
    g[v].push_back(u);
    e2i[PI(min(u, v), max(u, v))] = i;
  }
  
  BiconnectedComponentTree bct(g, e2i);
  bct.build();
  VI dep(bct.tree.size());
  dfs(0, bct.tree.size(), bct.tree, 0, dep);
  LowestCommonAncestor lca(bct.tree.size(), bct.tree);
  int q;
  cin >> q;
  REP(_, 0, q) {
    int x, y;
    cin >> x >> y;
    x--, y--;
    if (x == y) {
      cout << "0\n";
      continue;
    }
    int bx = bct.cmp_node[x];
    int by = bct.cmp_node[y];
    if (bx < 0) {
      int w = g[x][0];
      int e = e2i[PI(min(x, w), max(x, w))];
      bx = bct.cmp[e];
    }
    if (by < 0) {
      int w = g[y][0];
      int e = e2i[PI(min(y, w), max(y, w))];
      by = bct.cmp[e];
    }
    int l = lca.lca(bx, by);
    int dist = dep[bx] + dep[by] - 2 * dep[l];
    if (bct.cmp_node[x] >= 0) {
      dist--;
    }
    if (bct.cmp_node[y] >= 0) {
      dist--;
    }
    cout << dist / 2 << "\n";
  }
}
