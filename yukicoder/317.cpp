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

const int N = 100010;
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

const int B = 500;
int dp[B][N];
const int inf = 1e8;

void solve(const vector<PI> &bag) {
  assert (bag.size() < B);
  REP(i, 0, bag.size() + 1) {
    REP(j, 0, N) {
      dp[i][j] = inf;
    }
  }
  dp[0][0] = 0;
  REP(i, 1, bag.size() + 1) {
    PI cur = bag[i - 1];
    REP(j, 0, N) {
      int res = dp[i - 1][j];
      if (j >= cur.first) {
	res = min(res, dp[i - 1][j - cur.first] + cur.second);
      }
      dp[i][j] = res;
    }
  }
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  UnionFind uf(n);
  REP(i, 0, m) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    uf.unite(u, v);
  }
  VI pop(n, 0);
  REP(i, 0, n) {
    pop[uf.root(i)]++;
  }
  map<int, int> cons;
  REP(i, 0, n) {
    int s = pop[i];
    if (s != 0) {
      if (cons.count(s) == 0) {
	cons[s] = 0;
      }
      cons[s]++;
    }
  }
  vector<PI> bag;
  for (map<int, int>::iterator it = cons.begin(); it != cons.end(); ++it) {
    int p = it->first;
    int q = it->second;
    int v = 1;
    while (v < q) {
      bag.push_back(PI(p * v, v));
      q -= v;
      v *= 2;
    }
    if (q > 0) {
      bag.push_back(PI(p * q, q));
    }
  }
  solve(bag);
  REP(i, 1, n + 1) {
    int res = dp[bag.size()][i];
    cout << (res == inf ? -1 : res - 1) << "\n";
  }
}
