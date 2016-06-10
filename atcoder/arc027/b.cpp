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

int main(void){
  int n;
  string s1, s2;
  cin >> n >> s1 >> s2;
  UnionFind uf(26);
  int poss[26] = {};
  vector<bool> used(26, false);
  REP(i, 0, 26) {
    poss[i] = 0x3ff;
  }
  REP(i, 0, n) {
    if (s1[i] >= 'A' && s2[i] >= 'A') {
      uf.unite(s1[i] - 'A', s2[i] - 'A');
    }
    if (s1[i] >= 'A') {
      used[s1[i] - 'A'] = true;
    }
    if (s2[i] >= 'A') {
      used[s2[i] - 'A'] = true;
    }
  }
  REP(i, 0, n) {
    if (s1[i] >= 'A' ^ s2[i] >= 'A') {
      int x, c;
      if (s2[i] >= 'A') {
	swap(s1[i], s2[i]);
      }
      x = s2[i] - '0';
      c = s1[i] - 'A';
      int r = uf.root(c);
      poss[r] &= 1 << x;
      if (i == 0) {
	poss[r] &= ~1;
      }
    }
    if (i == 0) {
      if (s1[i] >= 'A') {
	int r = uf.root(s1[i] - 'A');
	poss[r] &= ~1;
      }
      if (s2[i] >= 'A') {
	int r = uf.root(s2[i] - 'A');
	poss[r] &= ~1;
      }
    }
  }
  REP(i, 0, 26) {
    used[i] = used[i] && (uf.root(i) == i);
  }
  ll cnt = 1;
  REP(i, 0, 26) {
    if (used[i]) {
      cnt *= __builtin_popcount(poss[i]);
    }
  }
  cout << cnt << endl;
}
