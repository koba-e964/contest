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

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
public:
  std::vector<int> disj;
  std::vector<int> rank;
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<PI, int> PPII;

const int N = 501000;
vector<PPII> es[N];
vector<PPII> es_cont[N];
int u[N], v[N], w[N];
int cont_u[N], cont_v[N];
bool defunct[N];
int used[N];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    cin >> u[i] >> v[i] >> w[i];
    u[i]--, v[i]--;
    es[w[i]].push_back(PPII(PI(u[i], v[i]), i));
  }
  UnionFind uf(n);
  REP(i, 1, N) {
    es_cont[i] = vector<PPII>(es[i].size());
    REP(j, 0, es[i].size()) {
      PPII d = es[i][j];
      PI uv = d.first;
      int x = uv.first;
      int y = uv.second;
      es_cont[i][j] = PPII(PI(uf.root(x), uf.root(y)), d.second);
      cont_u[d.second] = uf.root(x);
      cont_v[d.second] = uf.root(y);
      if (uf.is_same_set(x, y)) {
	defunct[d.second] = true;
      }
    }
    REP(j, 0, es[i].size()) {
      PPII d = es[i][j];
      PI uv = d.first;
      int x = uv.first;
      int y = uv.second;
      if (!uf.is_same_set(x, y)) {
        uf.unite(x, y);
	used[i]++;
      }
    }
  }
  int q;
  cin >> q;
  UnionFind tmp(n);
  REP(loop_cnt, 0, q) {
    int k;
    cin >> k;
    VI ee(k);
    REP(i, 0, k) {
      cin >> ee[i];
      ee[i]--;
    }
    bool ok = true;
    map<int, int> freq;
    set<int> touched;
    // Check whether S forms a cycle or not
    REP(i, 0, k) {
      // if ee[i] is defunct, die immediately
      if (defunct[ee[i]]) {
	ok = false;
	break;
      }
      int x = cont_u[ee[i]];
      int y = cont_v[ee[i]];
      if (DEBUG) {
	cerr << "edge " << ee[i] << " x = " << x << " y = " << y << endl;
      }
      if (tmp.is_same_set(x, y)) {
	ok = false;
	break;
      }
      touched.insert(x);
      touched.insert(y);
      tmp.unite(x, y);
      freq[w[ee[i]]] += 1;
    }
    for (auto ent: freq) {
      int ww = ent.first;
      int f = ent.second;
      if (used[ww] < f) {
	ok = false;
	break;
      }
    }
    cout << (ok ? "YES" : "NO") << "\n";
    for (auto x: touched) {
      tmp.disj[x] = x;
      tmp.rank[x] = 0;
    }
  }
}
