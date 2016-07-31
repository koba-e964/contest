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
typedef pair<int, int> PI;
const double EPS=1e-9;

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


int check(UnionFind &uf, const VI &wei, int x, int y) {
  int rx = uf.root(x);
  int ry = uf.root(y);
  if (rx == ry) {
    return wei[rx];
  } else {
    return wei[rx] + wei[ry];
  }
}

int main(void){
  int n, m, q;
  cin >> n >> m;
  vector<PI> edge(m);
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edge[i] = PI(a, b);
  }
  cin >> q;
  VI x(q), y(q), z(q);
  VI lo(q), hi(q);
  REP(i, 0, q) {
    cin >> x[i] >> y[i] >> z[i];
    x[i]--;
    y[i]--;
    lo[i] = -1;
    hi[i] = m;
  }
  REP(loop, 0, 20) {
    UnionFind uf(n);
    VI wei(n);
    REP(i, 0, n) {
      wei[i] = 1;
    }
    vector<PI> mid(q);
    REP(i, 0, q) {
      mid[i].first = (lo[i] + hi[i]) / 2;
      mid[i].second = i;
    }
    sort(mid.begin(), mid.end());
    int cur = 0;
    REP(i, 0, m) {
      int xx = edge[i].first;
      int yy = edge[i].second;
      if (uf.root(xx) != uf.root(yy)) {
	int rxx = uf.root(xx);
	int ryy = uf.root(yy);
	uf.unite(xx, yy);
	if (rxx == uf.root(xx)) {
	  wei[rxx] += wei[ryy];
	} else {
	  wei[ryy] += wei[rxx];
	}
      }
      while (cur < q && mid[cur].first <= i) {
	int qq = mid[cur].second;
	// check
	if (check(uf, wei, x[qq], y[qq]) >= z[qq]) {
	  hi[qq] = i;
	} else {
	  lo[qq] = i;
	}
	cur++;
      }
    }
  }
  REP(i, 0, q) {
    cout << hi[i] + 1 << endl;
  }
}
