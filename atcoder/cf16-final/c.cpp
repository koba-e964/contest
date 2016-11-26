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

const int N = 100100;
VI avail[N];

int main(void){
  int n, m;
  cin >> n >> m;
  UnionFind uf(n);
  VI man(m, -1);
  REP(i, 0, n) {
    int ki;
    cin >> ki;
    REP(j, 0, ki) {
      int l;
      cin >> l;
      l--;
      avail[i].push_back(l);
      if (man[l] == -1) {
	man[l] = i;
      } else {
	uf.unite(man[l], i);
      }
    }
  }
  VI root(n, 0);
  int numroot = 0;
  REP(i, 0, n) {
    int r = uf.root(i);
    if (root[r] == 0) {
      numroot++;
      root[r] = 1;
    }
  }
  cout << (numroot >= 2 ? "NO" : "YES") << endl;
}
