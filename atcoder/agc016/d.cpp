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
  cin >> n;
  VI a(n + 1), b(n + 1);
  REP(i, 0, n) {
    cin >> a[i];
    a[n] ^= a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
    b[n] ^= b[i];
  }
  {
    VI acp(a);
    VI bcp(b);
    sort(acp.begin(), acp.end());
    sort(bcp.begin(), bcp.end());
    if (acp != bcp) {
      cout << "-1" << endl;
      return 0;
    }
  }
  // coord comp
  set<int> comp;
  REP(i, 0, n + 1) {
    comp.insert(a[i]);
    comp.insert(b[i]);
  }
  vector<int> aux(comp.begin(), comp.end());
  sort(aux.begin(), aux.end());
  map<int, int> tbl;
  REP(i, 0, aux.size()) {
    tbl[aux[i]] = i;
  }
  REP(i, 0, n + 1) {
    a[i] = tbl[a[i]];
    b[i] = tbl[b[i]];
  }
  vector<PI> ops;
  vector<int> nop(n + 1, 0);
  UnionFind uf(n);
  REP(i, 0, n + 1) {
    if (a[i] != b[i]) {
      ops.push_back(PI(a[i], b[i]));
      uf.unite(a[i], b[i]);
    }
  }
  REP(i, 0, ops.size()) {
    nop[uf.root(ops[i].first)] += 1;
  }
  int tot = 0;
  int top = uf.root(b[n]);
  REP(i, 0, n + 1) {
    if (nop[i] > 0) {
      // cerr << "(" << i << ", " << nop[i] << ")" << endl;
      assert (nop[i] >= 2);
      bool flag = i == top;
      tot += flag ? nop[i] : nop[i] + 1;
    }
  }
  if (a[n] != b[n]) {
    tot -= 1;
  }
  cout << tot << endl;
}
