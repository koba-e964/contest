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
typedef pair<ll, ll> PL;

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


vector<pair<ll, PI> > calc(const VL &x, const VL &y) {
  ll tot = 0;
  int n = x.size();
  vector<PL> p(n);
  REP(i, 0, n) {
    p[i] = PL(x[i], i);
  }
  sort(p.begin(), p.end());
  vector<pair<ll, PI> > pool;
  REP(i, 0, n - 1) {
    int u = p[i].second;
    int v = p[i + 1].second;
    ll dist = min(abs(x[u] - x[v]), abs(y[u] - y[v]));
    pool.push_back(make_pair(dist, PI(u, v)));
  }
  return pool;
}

int main(void){
  int n;
  cin >> n;
  VL x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  vector<pair<ll, PI> > pool = calc(x, y);
  {
    vector<pair<ll, PI> > pool2 = calc(y, x);
    REP(i, 0, pool2.size()) {
      pool.push_back(pool2[i]);
    }
  }
  sort(pool.begin(), pool.end());
  UnionFind uf(n);
  ll tot = 0;
  REP(i, 0, pool.size()) {
    pair<ll, PI> t = pool[i];
    int u = t.second.first;
    int v = t.second.second;
    if (uf.is_same_set(u, v)) {
      continue;
    }
    uf.unite(u, v);
    tot += t.first;
  }
  cout << tot << endl;
}
