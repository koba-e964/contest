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


int gcd(int x, int y) {
  while (y != 0) {
    int r = x % y;
    x = y;
    y = r;
  }
  return x;
}


const int B = 46;


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll k;
  cin >> k;
  while (gcd(k, 10) != 1) {
    k /= gcd(k, 10);
  }
  UnionFind uf(k);
  REP(i, 0, k) {
    uf.unite(i, (10 * i) % k);
  }
  vector<vector<bool> > dp(B, vector<bool>(k, false));
  dp[0][0] = true;
  REP(u, 1, B) {
    REP(b, 1, 10) {
      int d = k * b % 10;
      int c = k * b / 10;
      if (u >= d) {
	REP(i, 0, k) {
	  int teni = uf.root(i);
	  dp[u][teni] = dp[u][teni] || dp[u - d][uf.root((i + c) % k)];
	}
      }
    }
    if (0) {
      cerr << "u = " << u << endl;
      cerr << "dp";
      REP(i, 0, k) {
	if (uf.root(i) == i) {
	  cerr << i << ": " << dp[u][i] << endl;
	}
      }
      cerr << endl;
    }
  }
  int mi = 1e8;
  REP(u, 1, B) {
    if (dp[u][0]) {
      mi = min(mi, u);
    }
  }
  cout << mi << "\n";
}
