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


const int M = 10010;
int n, m;
int a[M], b[M], c[M], t[M];

typedef pair<double, int> PDI;

bool check(double hourly) {
  vector<PDI> edges(m);
  REP(i, 0, m) {
    edges[i] = PDI(-t[i] * hourly + c[i], i);
  }
  sort(edges.begin(), edges.end());
  UnionFind uf(n);
  double tot = 0;
  int cc = n;
  int pos = 0;
  while (pos < m && cc > 1) {
    PDI e = edges[pos];
    int idx = e.second;
    if (e.first < 0 || not uf.is_same_set(a[idx], b[idx])) {
      if (not uf.is_same_set(a[idx], b[idx])) {
	cc--;
      }
      uf.unite(a[idx], b[idx]);
      tot += e.first;
    }
    pos++;
  }
  assert (cc == 1);
  return tot <= 0;
}


int main(void){
  cin >> n >> m;
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> c[i] >> t[i];
  }
  double lo = 0.0, hi = 1.1e6;
  REP(loop_cnt, 0, 60) {
    double mid = (hi + lo) / 2;
    if (not check(mid)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  printf("%.9f\n", lo);
}
