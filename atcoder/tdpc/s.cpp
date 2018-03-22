#include <algorithm>
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

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
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


using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int H = 6;
int h, w;


pair<VI, int> next_conn(int whole, const VI &pre, int bits, int idx) {
  UnionFind uf(2 * h);
  int prim = pre[idx];
  REP(i, 0, h) {
    if ((whole & 1 << i) && (bits & 1 << i)) {
      uf.unite(i, i + h);
    }
  }
  for (auto pat: pre) {
    REP(i, 0, h) {
      if ((pat & 1 << i) == 0) continue;
      REP(j, 0, i) {
	if ((pat & 1 << j) == 0) continue;
	uf.unite(i, j);
      }
    }
  }
  REP(i, 0, h) {
    if ((bits & 1 << i) == 0) continue;
    if ((bits & 1 << (i + 1)) == 0) continue;
    uf.unite(i + h, i + 1 + h);
  }
  UnionFind down(h);
  REP(i, 0, h) {
    REP(j, 0, h) {
      if (uf.is_same_set(i + h, j + h)) down.unite(i, j);
    }
  }
  VI conn(h);
  REP(i, 0, h) {
    if ((bits & 1 << i) == 0) continue;
    conn[down.root(i)] |= 1 << i;
  }
  VI ret;
  REP(i, 0, h) if (conn[i] != 0) ret.push_back(conn[i]);
  sort(ret.begin(), ret.end());
  int retidx = -1;
  int lowest = -1;
  REP(i, 0, h) {
    if (prim & 1 << i) {
      lowest = i;
      break;
    }
  }
  REP(i, 0, ret.size()) {
    int lw = -1;
    REP(j, 0, h) {
      if (ret[i] & 1 << j) {
	lw = j;
	break;
      }
    }
    if (uf.is_same_set(lowest, lw + h)) {
      retidx = i;
      break;
    }
  }
  return make_pair(ret, retidx);
}

map<VI, ll> dp[101][1 << H][H];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> h >> w;
#if 0
  memset(tbl, -1, sizeof(tbl));
  REP(i, 0, 1 << h) {
    REP(j, 0, 1 << h) {
      UnionFind uf(2 * h);
      REP(k, 0, h) {
	if ((i & 1 << k) && (j & 1 << k)) {
	  uf.unite(k, k + h);
	}
      }
      REP(k, 0, h - 1) {
	if ((i & 3 << k) == 3 << k) {
	  uf.unite(k, k + 1);
	}
	if ((j & 3 << k) == 3 << k) {
	  uf.unite(k + h, k + h + 1);
	}
      }
      REP(k, 0, i + 1) {
	if ((i & k) != k) continue;
	int res = 0;
	REP(l, 0, h) {
	  if ((k & 1 << l) == 0) continue;
	  REP(u, 0, h) {
	    if (uf.is_same_set(l, h + u)) {
	      res |= 1 << u;
	    }
	  }
	}
	tbl[i][j][k] = res;
      }
    }
  }
#endif
  dp[0][1][0][VI(1, 1)] = 1;
  REP(i, 0, w) {
    REP(a, 0, 1 << h) {
      REP(b, 0, h) {
	for (auto &e: dp[i][a][b]) {
	  const VI &key = e.first;
	  ll val = e.second;
	  if (DEBUG) {
	    cerr << "dp " << i << " " << a << " " << b << "--";
	    REP(l, 0, key.size()) {
	      cerr << " " << key[l];
	    }
	    cerr << " => ";
	    cerr << val << endl;
	    cerr << endl;
	  }
	  REP(c, 0, 1 << h) {
	    pair<VI, int> nxt_conn = next_conn(a, key, c, b);
	    if (nxt_conn.second >= 0) {
	      add(dp[i + 1][c][nxt_conn.second][nxt_conn.first], val);
	    }
	  }
	}
      }
    }
  }
  ll tot = 0;
  REP(i, 1 << (h - 1), 1 << h) {
    REP(b, 0, h) {
      for (auto &e: dp[w][i][b]) {
	if (e.first[b] & 1 << (h - 1))
	  add(tot, e.second);
      }
    }
  }
  cout << tot << endl;
}
