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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  vector<PI> sorted(n);
  REP(i, 0, n) {
    sorted[i] = PI(a[i], i);
  }
  sort(sorted.begin(), sorted.end());
  UnionFind uf(n);
  REP(i, 0, n) {
    uf.unite(i, sorted[i].second);
  }
  vector<VI> pool(n);
  REP(i, 0, n) {
    int r = uf.root(i);
    pool[r].push_back(i);
  }
  int cnt = 0;
  REP(i, 0, n) {
    cnt += pool[i].size() != 0;
  }
  cout << cnt << endl;
  REP(i, 0, n) {
    if (pool[i].size() == 0) {
      continue;
    }
    cout << pool[i].size();
    REP(j, 0, pool[i].size()) {
      cout << " " << pool[i][j] + 1;
    }
    cout << endl;
  }
}
