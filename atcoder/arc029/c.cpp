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
const int N = 100010;
const int M = 200010;
int n, m;
int c[N];
int a[M], b[M];
int r[M];

int main(void){
  cin >> n >> m;
  vector<PI> pool;
  REP(i, 0, n) {
    cin >> c[i];
    pool.push_back(PI(c[i], i));
  }
  REP(i, 0, m) {
    cin >> a[i] >> b[i] >> r[i];
    a[i]--, b[i]--;
    pool.push_back(PI(r[i], i + n));
  }
  sort(pool.begin(), pool.end());
  UnionFind uf(n + 1);
  int cnt = 0;
  ll cost = 0;
  REP(i, 0, n + m) {
    PI act = pool[i];
    if (act.second < n) {
      // adding
      if (! uf.is_same_set(0, act.second + 1)) {
	uf.unite(0, act.second + 1);
	cnt++;
	cost += act.first;
      }
    } else {
      int e = act.second - n;
      if (!uf.is_same_set(a[e] + 1, b[e] + 1)) {
	uf.unite(a[e] + 1, b[e] + 1);
	cnt++;
	cost += r[e];
      }
    }
  }
  assert (cnt == n);
  cout << cost << endl;
}
