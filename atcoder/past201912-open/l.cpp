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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

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

int n, m;
const int N = 35;
double x[N], y[N], c[N];
double cst[N][N];

double sq(double x) { return x * x; }

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  cin >> n >> m;
  REP(i, 0, n) cin >> x[i] >> y[i] >> c[i];
  REP(i, 0, m) cin >> x[n + i] >> y[n + i] >> c[n + i];
  REP(i, 0, n + m) {
    REP(j, 0, n + m) {
      double d = sq(x[i] - x[j]) + sq(y[i] - y[j]);
      d = sqrt(d);
      if (c[i] != c[j]) {
        d *= 10;
      }
      cst[i][j] = d;
    }
  }
  double mi = 1e18;
  REP(bits, 0, 1 << m) {
    VI vs;
    REP(i, 0, n) vs.push_back(i);
    REP(i, 0, m) if (bits & 1 << i) vs.push_back(n + i);
    vector<pair<double, PI> > e;
    int len = vs.size();
    REP(i, 0, len) REP(j, 0, i) {
      e.push_back(make_pair(cst[vs[i]][vs[j]], PI(i, j)));
    }
    sort(e.begin(), e.end());
    UnionFind uf(len);
    double tot = 0;
    for (auto k: e) {
      int x = k.second.first, y = k.second.second;
      if (uf.is_same_set(x, y)) continue;
      tot += k.first;
      uf.unite(x, y);
    }
    mi = min(mi, tot);
  }
  cout << fixed << setprecision(15) << mi << endl;
}
